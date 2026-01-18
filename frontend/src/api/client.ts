import { InvariantResult, MonitorStatus, HealthResponse } from '../types/invariant';

// Fallback to production URL if env var not set
const API_BASE = import.meta.env.VITE_API_URL || 'http://n2.ckey.vn:1444';

async function fetchWithError<T>(url: string): Promise<T> {
    const res = await fetch(url);
    if (!res.ok) {
        throw new Error(`API error: ${res.status} ${res.statusText}`);
    }
    return res.json();
}

export async function fetchInvariants(): Promise<InvariantResult[]> {
    return fetchWithError<InvariantResult[]>(`${API_BASE}/api/invariants`);
}

export async function fetchInvariant(id: string): Promise<InvariantResult> {
    return fetchWithError<InvariantResult>(`${API_BASE}/api/invariants/${id}`);
}

export async function fetchStatus(): Promise<MonitorStatus> {
    return fetchWithError<MonitorStatus>(`${API_BASE}/api/status`);
}

export async function fetchHealth(): Promise<HealthResponse> {
    return fetchWithError<HealthResponse>(`${API_BASE}/health`);
}

export interface MonitorRequest {
    object_id: string;
    network?: string;
}

export interface MonitorResponse {
    success: boolean;
    message: string;
    object_id: string;
    object_type?: string;
}

export async function addMonitoredObject(request: MonitorRequest): Promise<MonitorResponse> {
    const res = await fetch(`${API_BASE}/api/monitor`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
    });

    if (!res.ok) {
        throw new Error(`API error: ${res.status} ${res.statusText}`);
    }

    return res.json();
}

// ===== AI Analysis Types =====

export type LlmProvider = 'openrouter' | 'ollama';

export interface AnalyzeRequest {
    package_id: string;
    module_name?: string;
    llm_provider: LlmProvider;
    api_key?: string;
    model: string;
    ollama_url?: string;
    network?: string;  // "mainnet" or "testnet"
}

export interface FieldMetadata {
    name: string;
    type_: string;
}

export interface StructMetadata {
    name: string;
    abilities: string[];
    fields: FieldMetadata[];
}

export interface ModuleMetadata {
    package_id: string;
    module_name: string;
    structs: StructMetadata[];
    functions: Array<{
        name: string;
        visibility: string;
        is_entry: boolean;
        parameters: string[];
        return_types: string[];
    }>;
}

export interface SuggestedInvariant {
    id: string;
    name: string;
    description: string;
    formula: string;
    severity: string;
    fields_used: string[];
}

export interface AnalysisResult {
    package_id: string;
    module_name: string;
    suggested_invariants: SuggestedInvariant[];
    analysis_notes: string;
}

export interface AnalyzeResponse {
    success: boolean;
    message: string;
    modules: ModuleMetadata[];
    analysis_results: AnalysisResult[];
}

export async function analyzePackage(request: AnalyzeRequest): Promise<AnalyzeResponse> {
    const res = await fetch(`${API_BASE}/api/analyze`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
    });

    if (!res.ok) {
        throw new Error(`API error: ${res.status} ${res.statusText}`);
    }

    return res.json();
}

export async function fetchModuleMetadata(packageId: string, moduleName: string): Promise<ModuleMetadata> {
    return fetchWithError<ModuleMetadata>(`${API_BASE}/api/metadata/${packageId}/${moduleName}`);
}

// ===== Add Suggested Invariants =====

export interface AddInvariantsRequest {
    invariants: SuggestedInvariant[];
    package_id: string;
    module_name: string;
}

export interface AddInvariantsResponse {
    success: boolean;
    message: string;
    added_count: number;
}

export async function addSuggestedInvariants(request: AddInvariantsRequest): Promise<AddInvariantsResponse> {
    const res = await fetch(`${API_BASE}/api/invariants/add`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
    });

    if (!res.ok) {
        throw new Error(`API error: ${res.status} ${res.statusText}`);
    }

    return res.json();
}

// ===== Remove Invariant =====

export interface RemoveInvariantRequest {
    invariant_id: string;
}

export interface RemoveInvariantResponse {
    success: boolean;
    message: string;
}

export async function removeInvariant(request: RemoveInvariantRequest): Promise<RemoveInvariantResponse> {
    const res = await fetch(`${API_BASE}/api/invariants/remove`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
    });

    if (!res.ok) {
        throw new Error(`API error: ${res.status} ${res.statusText}`);
    }

    return res.json();
}
