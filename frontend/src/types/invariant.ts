export type InvariantStatus = 'Ok' | 'Violated' | 'Error';

export interface InvariantComputation {
    inputs: Record<string, string>;
    formula: string;
    result: string;
}

export interface InvariantResult {
    id: string;
    name: string;
    description: string;
    status: InvariantStatus;
    evaluated_at: string;
    computation: InvariantComputation;
    violation_reason: string | null;
}

export interface MonitorStatus {
    last_check: string | null;
    violations: number;
    total_invariants: number;
    all_ok: boolean;
    monitored_objects: string[];
}

export interface HealthResponse {
    status: string;
    uptime_secs: number;
}
