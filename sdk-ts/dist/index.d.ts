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
    getListTaskTypes(): Promise<string[]>;
    getPayloadSchema(task_type: string): Promise<TaskPayloadSchema>;
    waitForResult(id: UUID, opts?: WaitOptions): Promise<Task>;
}
export type FieldType = "string" | "array" | "number" | "object" | "boolean";
export interface TaskPayloadSchema {
    task_type: string;
    description: string;
    fields: Record<string, PayloadField>;
}
export interface PayloadField {
    field_type: FieldType;
    required: boolean;
    description?: string;
    example?: unknown;
}
