

import fetch from "cross-fetch";

export type UUID = string;

export type TaskStatus = "Pending" | "Running" | "Completed" | "Failed";

export interface Task {
    id: UUID,
    task_type: string,
    payload: unknown,
    status: TaskStatus,
    result: TaskResult | null,
    created_at?: string;
    started_at?: string | null;
    completed_at?: string | null;
}
export interface TaskResult {
    output?: unknown | null;
    error?: string | null;

}


// request /response dtos

export interface CreateTaskRequest {
    task_type: string;
    payload: unknown;


}

export interface CreateTaskResponse extends Task { }


export interface SubmitResultRequest {
    output?: unknown | null;
    error?: string | null;
}

export interface SubmitResultResponse {
    status: TaskStatus;
}


export interface ClientOptions {
    baseUrl: string;
    timeoutMs?: number;
    headers?: Record<string, string>
}

export interface WaitOptions {
    initialDelayMs?: number; // first poll delay
    maxDelayMs?: number; // max backoff
    factor?: number; // exponential factor
    timeoutMs?: number; // overall timeout
}

function timeoutSignal(timeoutMs?: number): AbortSignal | undefined {
    if (!timeoutMs) return undefined;
    const controller = new AbortController();
    const id = setTimeout(() => controller.abort(), timeoutMs);
    controller.signal.addEventListener("abort", () => clearTimeout(id))
    return controller.signal;
}

export class Client {
    private baseUrl: string;
    private headers: Record<string, string>;
    private timeoutMs?: number;
    constructor(opts: ClientOptions) {
        this.baseUrl = opts.baseUrl.replace(/\/+$/, "");
        this.timeoutMs = opts.timeoutMs
        this.headers = opts.headers ?? { "Content-Type": "application/json" }

    }
    private url(path: string) {
        return `${this.baseUrl}${path}`
    }

    private async doFetch<T>(input: string, init: RequestInit = {}): Promise<T> {
        const controller = new AbortController();
        const signal = controller.signal;
        const finalSignal = init.signal ?? signal;

        if (this.timeoutMs) {
            const timeoutId = setTimeout(() => controller.abort(), this.timeoutMs);
            try {
                const res = await fetch(input, { ...init, signal: finalSignal })
                clearTimeout(timeoutId);
                if (!res.ok) throw new Error(`HTTP ${res.status} ${res.statusText}`);
                const json = (await res.json()) as T;
                return json;
            } catch (err) {
                clearTimeout(timeoutId);
                throw err;
            }
        }
        else {
            const res = await fetch(input, { ...init, signal: finalSignal });
            if (!res.ok) throw new Error(`HTTP ${res.status} ${res.statusText}`);
            return (await res.json()) as T;
        }
    }

    async createTask(task_type: string, payload: unknown): Promise<CreateTaskResponse> {
        const body: CreateTaskRequest = { task_type, payload };

        const url = this.url(`/task`);
        return await this.doFetch<CreateTaskResponse>(url, {
            method: "POST",
            body: JSON.stringify(body),
            headers: this.headers, 
        })
    }

    async getTask(id: UUID): Promise<Task> {
        const url = this.url(`/task/${id}`);
        return await this.doFetch<Task>(url, { method: "GET", headers: this.headers })
    }

    async getListTaskTypes(): Promise<string[]>{
        const url = this.url("/task/types");
        
        return await this.doFetch<string[]>(url , {
            method : "GET" , headers : this.headers
        });
    }
    async getPayloadSchema(task_type : string): Promise<TaskPayloadSchema>{
        const url = this.url(`/task/${task_type}/schema`);
        
        return await this.doFetch<TaskPayloadSchema>(url , {
            method : "GET" , headers : this.headers
        });
    }

    async waitForResult(id: UUID, opts?: WaitOptions): Promise<Task> {
        const initialDelayMs = opts?.initialDelayMs ?? 500;
        const factor = opts?.factor ?? 1.5;
        const maxDelayMs = opts?.maxDelayMs ?? 5000;
        const timeoutMs = opts?.timeoutMs;

        const start = Date.now();
        let attempt = 0;
        let delay = initialDelayMs;

        while (true) {
            const task = await this.getTask(id);
            if (task.status === "Completed" || task.status === "Failed") return task;

            if (timeoutMs && Date.now() - start > timeoutMs) {
                throw new Error("Wait for result : timeout")
            }

            await new Promise((resolve) => setTimeout(resolve, delay));
            attempt += 1;
            delay = Math.min(maxDelayMs, Math.round(delay * factor));
        }
    }

}

export type FieldType = "string" | "array" | "number" | "object" |"boolean"

export interface TaskPayloadSchema {
    task_type : string , 
    description : string , 
    fields : Record<string , PayloadField>
}

export interface PayloadField{
    field_type : FieldType , 
    required : boolean , 
    description ?: string ,
    example ?: unknown
}