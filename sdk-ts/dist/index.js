import fetch from "cross-fetch";
function timeoutSignal(timeoutMs) {
    if (!timeoutMs)
        return undefined;
    const controller = new AbortController();
    const id = setTimeout(() => controller.abort(), timeoutMs);
    controller.signal.addEventListener("abort", () => clearTimeout(id));
    return controller.signal;
}
export class Client {
    constructor(opts) {
        this.baseUrl = opts.baseUrl.replace(/\/+$/, "");
        this.timeoutMs = opts.timeoutMs;
        this.headers = opts.headers ?? { "Content-Type": "application/json" };
    }
    url(path) {
        return `${this.baseUrl}${path}`;
    }
    async doFetch(input, init = {}) {
        const controller = new AbortController();
        const signal = controller.signal;
        const finalSignal = init.signal ?? signal;
        if (this.timeoutMs) {
            const timeoutId = setTimeout(() => controller.abort(), this.timeoutMs);
            try {
                const res = await fetch(input, { ...init, signal: finalSignal });
                clearTimeout(timeoutId);
                if (!res.ok)
                    throw new Error(`HTTP ${res.status} ${res.statusText}`);
                const json = (await res.json());
                return json;
            }
            catch (err) {
                clearTimeout(timeoutId);
                throw err;
            }
        }
        else {
            const res = await fetch(input, { ...init, signal: finalSignal });
            if (!res.ok)
                throw new Error(`HTTP ${res.status} ${res.statusText}`);
            return (await res.json());
        }
    }
    async createTask(task_type, payload) {
        const body = { task_type, payload };
        const url = this.url(`/task`);
        return await this.doFetch(url, {
            method: "POST",
            body: JSON.stringify(body),
            headers: this.headers,
        });
    }
    async getTask(id) {
        const url = this.url(`/task/${id}`);
        return await this.doFetch(url, { method: "GET", headers: this.headers });
    }
    async getListTaskTypes() {
        const url = this.url("/task/types");
        return await this.doFetch(url, {
            method: "GET", headers: this.headers
        });
    }
    async getPayloadSchema(task_type) {
        const url = this.url(`/task/types/${task_type}/schema`);
        return await this.doFetch(url, {
            method: "GET", headers: this.headers
        });
    }
    async waitForResult(id, opts) {
        const initialDelayMs = opts?.initialDelayMs ?? 500;
        const factor = opts?.factor ?? 1.5;
        const maxDelayMs = opts?.maxDelayMs ?? 5000;
        const timeoutMs = opts?.timeoutMs;
        const start = Date.now();
        let attempt = 0;
        let delay = initialDelayMs;
        while (true) {
            const task = await this.getTask(id);
            if (task.status === "Completed" || task.status === "Failed")
                return task;
            if (timeoutMs && Date.now() - start > timeoutMs) {
                throw new Error("Wait for result : timeout");
            }
            await new Promise((resolve) => setTimeout(resolve, delay));
            attempt += 1;
            delay = Math.min(maxDelayMs, Math.round(delay * factor));
        }
    }
}
