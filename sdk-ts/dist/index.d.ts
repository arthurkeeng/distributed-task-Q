export type UUID = string;
export type TaskStatus = "Pending" | "Running" | "Completed" | "Failed";
export interface Task {
    id: UUID;
    task_type: string;
    payload: unknown;
    status: TaskStatus;
    result: TaskResult | null;
    created_at?: string;
    started_at?: string | null;
    completed_at?: string | null;
}
export interface TaskResult {
    output?: unknown | null;
    error?: string | null;
}
export interface CreateTaskRequest {
    task_type: string;
    payload: unknown;
}
export interface CreateTaskResponse extends Task {
}
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
    headers?: Record<string, string>;
}
export interface WaitOptions {
    initialDelayMs?: number;
    maxDelayMs?: number;
    factor?: number;
    timeoutMs?: number;
}
export declare class Client {
    private baseUrl;
    private headers;
    private timeoutMs?;
    constructor(opts: ClientOptions);
    private url;
    private doFetch;
    createTask(task_type: string, payload: unknown): Promise<CreateTaskResponse>;
    getTask(id: UUID): Promise<Task>;
    submitResult(id: UUID, result: SubmitResultRequest): Promise<SubmitResultResponse>;
    waitForResult(id: UUID, opts?: WaitOptions): Promise<Task>;
}
