import { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { analyzePackage, addSuggestedInvariants, LlmProvider, AnalyzeResponse, SuggestedInvariant } from '../api/client';
import { useNetwork } from '../context/NetworkContext';

const OPENROUTER_MODELS = [
    // Paid models
    { value: 'anthropic/claude-3.5-sonnet', label: 'Claude 3.5 Sonnet (Paid)' },
    { value: 'openai/gpt-4o', label: 'GPT-4o (Paid)' },
    { value: 'google/gemini-pro-1.5', label: 'Gemini Pro 1.5 (Paid)' },
    { value: 'meta-llama/llama-3.1-70b-instruct', label: 'Llama 3.1 70B (Paid)' },
    // Free models
    { value: 'google/gemini-flash-1.5', label: 'Gemini Flash 1.5 (Free)' },
    { value: 'meta-llama/llama-3.2-11b-vision-instruct:free', label: 'Llama 3.2 11B Vision (Free)' },
];

const SUGGESTED_OLLAMA_MODELS = [
    { value: 'llama3.2', label: 'Llama 3.2 (Suggested)' },
    { value: 'llama3.1', label: 'Llama 3.1 (Suggested)' },
    { value: 'codellama', label: 'CodeLlama (Suggested)' },
    { value: 'mistral', label: 'Mistral (Suggested)' },
    { value: 'qwen2.5-coder', label: 'Qwen 2.5 Coder (Suggested)' },
];

interface Props {
    onAnalysisComplete?: (result: AnalyzeResponse) => void;
}

interface OllamaModel {
    name: string;
    size: number;
    modified_at: string;
}

export function AnalyzeContractForm({ onAnalysisComplete }: Props) {
    const { network } = useNetwork();
    const [packageId, setPackageId] = useState('');
    const [moduleName, setModuleName] = useState('');
    const [provider, setProvider] = useState<LlmProvider>('ollama');
    const [apiKey, setApiKey] = useState('');
    const [model, setModel] = useState('llama3.2');
    const [ollamaUrl, setOllamaUrl] = useState('http://localhost:11434');
    const [showSettings, setShowSettings] = useState(false);
    const [result, setResult] = useState<AnalyzeResponse | null>(null);
    const [localModels, setLocalModels] = useState<OllamaModel[]>([]);
    const [loadingModels, setLoadingModels] = useState(false);

    const mutation = useMutation({
        mutationFn: analyzePackage,
        onSuccess: (data) => {
            setResult(data);
            onAnalysisComplete?.(data);
        },
    });

    // Fetch local Ollama models
    const fetchLocalModels = async () => {
        setLoadingModels(true);
        try {
            const res = await fetch(`${ollamaUrl}/api/tags`);
            if (res.ok) {
                const data = await res.json();
                setLocalModels(data.models || []);
            }
        } catch (err) {
            console.error('Failed to fetch Ollama models:', err);
            setLocalModels([]);
        } finally {
            setLoadingModels(false);
        }
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();

        mutation.mutate({
            package_id: packageId.trim(),
            module_name: moduleName.trim() || undefined,
            llm_provider: provider,
            api_key: provider === 'openrouter' ? apiKey : undefined,
            model,
            ollama_url: provider === 'ollama' ? ollamaUrl : undefined,
            network: network,  // Pass selected network from context
        });
    };

    const handleProviderChange = (newProvider: LlmProvider) => {
        setProvider(newProvider);
        if (newProvider === 'ollama') {
            setModel('llama3.2');
            fetchLocalModels();
        } else {
            setModel('anthropic/claude-3.5-sonnet');
        }
    };

    // Combine suggested and local models for Ollama
    const ollamaModels = provider === 'ollama'
        ? [
            ...SUGGESTED_OLLAMA_MODELS,
            ...localModels
                .filter(m => !SUGGESTED_OLLAMA_MODELS.some(s => s.value === m.name))
                .map(m => ({ value: m.name, label: `${m.name} (Local)` }))
        ]
        : [];

    const models = provider === 'openrouter' ? OPENROUTER_MODELS : ollamaModels;

    return (
        <div className="analyze-form">
            <h3 className="form-title">AI Contract Analysis</h3>
            <p className="form-description">
                Analyze any Sui package on {network} to auto-generate safety invariants
            </p>

            <form onSubmit={handleSubmit} className="form-content">
                <div className="input-group">
                    <input
                        type="text"
                        value={packageId}
                        onChange={(e) => setPackageId(e.target.value)}
                        placeholder="Package ID (0x...)"
                        className="text-input"
                        disabled={mutation.isPending}
                    />
                </div>

                <div className="input-group">
                    <input
                        type="text"
                        value={moduleName}
                        onChange={(e) => setModuleName(e.target.value)}
                        placeholder="Module name (optional)"
                        className="text-input"
                        disabled={mutation.isPending}
                    />
                </div>

                <button
                    type="button"
                    className="settings-toggle"
                    onClick={() => setShowSettings(!showSettings)}
                >
                    <span>LLM Settings</span>
                    <span>{showSettings ? '−' : '+'}</span>
                </button>

                {showSettings && (
                    <div className="llm-settings">
                        <div className="setting-group">
                            <label>Provider</label>
                            <div className="provider-buttons">
                                <button
                                    type="button"
                                    className={`provider-btn ${provider === 'ollama' ? 'active' : ''}`}
                                    onClick={() => handleProviderChange('ollama')}
                                >
                                    Ollama (Local)
                                </button>
                                <button
                                    type="button"
                                    className={`provider-btn ${provider === 'openrouter' ? 'active' : ''}`}
                                    onClick={() => handleProviderChange('openrouter')}
                                >
                                    OpenRouter
                                </button>
                            </div>
                        </div>

                        {provider === 'openrouter' && (
                            <div className="setting-group">
                                <label>API Key</label>
                                <input
                                    type="password"
                                    value={apiKey}
                                    onChange={(e) => setApiKey(e.target.value)}
                                    placeholder="sk-or-..."
                                    className="text-input"
                                />
                                <a
                                    href="https://openrouter.ai/keys"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    className="setting-hint"
                                >
                                    Get API key →
                                </a>
                            </div>
                        )}

                        {provider === 'ollama' && (
                            <div className="setting-group">
                                <label>Ollama URL</label>
                                <div className="input-group">
                                    <input
                                        type="text"
                                        value={ollamaUrl}
                                        onChange={(e) => setOllamaUrl(e.target.value)}
                                        placeholder="http://localhost:11434"
                                        className="text-input"
                                    />
                                    <button
                                        type="button"
                                        className="btn btn-secondary"
                                        onClick={fetchLocalModels}
                                        disabled={loadingModels}
                                    >
                                        {loadingModels ? '...' : '↻'}
                                    </button>
                                </div>
                                {localModels.length > 0 && (
                                    <span className="setting-hint">
                                        Found {localModels.length} local model(s)
                                    </span>
                                )}
                            </div>
                        )}

                        <div className="setting-group">
                            <label>Model</label>
                            <select
                                value={model}
                                onChange={(e) => setModel(e.target.value)}
                                className="model-select"
                            >
                                {models.map((m) => (
                                    <option key={m.value} value={m.value}>
                                        {m.label}
                                    </option>
                                ))}
                            </select>
                        </div>
                    </div>
                )}

                <button
                    type="submit"
                    className="btn btn-primary analyze-button"
                    disabled={mutation.isPending || !packageId.trim()}
                >
                    {mutation.isPending ? 'Analyzing...' : 'Analyze Contract'}
                </button>

                {mutation.isError && (
                    <div className="form-message error">✕ {mutation.error.message}</div>
                )}
            </form>

            {result && result.success && (
                <div className="analysis-results">
                    <h4>Analysis Results</h4>
                    <p className="result-summary">{result.message}</p>

                    {result.analysis_results.map((ar) => (
                        <div key={ar.module_name} className="module-result">
                            <h5>{ar.module_name}</h5>

                            {ar.analysis_notes && (
                                <p className="analysis-notes">{ar.analysis_notes}</p>
                            )}

                            {/* Add All Button */}
                            {ar.suggested_invariants.length > 0 && (
                                <ModuleAddAllButton
                                    invariants={ar.suggested_invariants}
                                    packageId={ar.package_id}
                                    moduleName={ar.module_name}
                                />
                            )}

                            <div className="invariant-suggestions">
                                {ar.suggested_invariants.map((inv) => (
                                    <SuggestedInvariantCard
                                        key={inv.id}
                                        invariant={inv}
                                        packageId={ar.package_id}
                                        moduleName={ar.module_name}
                                    />
                                ))}
                            </div>
                        </div>
                    ))}

                    {result.modules.length > 0 && (
                        <details className="metadata-details">
                            <summary>View Module Metadata</summary>
                            <pre>{JSON.stringify(result.modules, null, 2)}</pre>
                        </details>
                    )}
                </div>
            )}
        </div>
    );
}

function ModuleAddAllButton({
    invariants,
    packageId,
    moduleName
}: {
    invariants: SuggestedInvariant[];
    packageId: string;
    moduleName: string;
}) {
    const queryClient = useQueryClient();
    const [allAdded, setAllAdded] = useState(false);

    const addAllMutation = useMutation({
        mutationFn: () => addSuggestedInvariants({
            invariants,
            package_id: packageId,
            module_name: moduleName,
        }),
        onSuccess: () => {
            setAllAdded(true);
            queryClient.invalidateQueries({ queryKey: ['invariants'] });
            queryClient.invalidateQueries({ queryKey: ['status'] });
        },
    });

    return (
        <button
            className={`btn ${allAdded ? 'btn-secondary' : 'btn-primary'}`}
            onClick={() => addAllMutation.mutate()}
            disabled={addAllMutation.isPending || allAdded}
            style={{ marginBottom: '16px', fontSize: '13px', padding: '10px 16px' }}
        >
            {allAdded
                ? `✓ All ${invariants.length} Invariants Added`
                : addAllMutation.isPending
                    ? 'Adding All...'
                    : `+ Add All ${invariants.length} to Monitoring`}
        </button>
    );
}

function SuggestedInvariantCard({
    invariant,
    packageId,
    moduleName
}: {
    invariant: SuggestedInvariant;
    packageId: string;
    moduleName: string;
}) {
    const queryClient = useQueryClient();
    const [isAdded, setIsAdded] = useState(false);

    const addMutation = useMutation({
        mutationFn: () => addSuggestedInvariants({
            invariants: [invariant],
            package_id: packageId,
            module_name: moduleName,
        }),
        onSuccess: () => {
            setIsAdded(true);
            queryClient.invalidateQueries({ queryKey: ['invariants'] });
            queryClient.invalidateQueries({ queryKey: ['status'] });
        },
    });

    const severityColors: Record<string, string> = {
        critical: '#ef4444',
        high: '#f59e0b',
        medium: '#eab308',
        low: '#22c55e',
    };

    return (
        <div className="suggested-invariant">
            <div className="suggested-invariant__header">
                <span className="suggested-invariant__id">{invariant.id}</span>
                <span
                    className="suggested-invariant__severity"
                    style={{ color: severityColors[invariant.severity] || '#9e9e9e' }}
                >
                    {invariant.severity}
                </span>
            </div>
            <div className="suggested-invariant__name">{invariant.name}</div>
            <div className="suggested-invariant__description">{invariant.description}</div>
            <code className="suggested-invariant__formula">{invariant.formula}</code>
            <div className="suggested-invariant__fields">
                Fields: {invariant.fields_used.join(', ')}
            </div>

            <button
                className={`btn ${isAdded ? 'btn-secondary' : 'btn-primary'}`}
                onClick={() => addMutation.mutate()}
                disabled={addMutation.isPending || isAdded}
                style={{ marginTop: '12px', width: '100%', fontSize: '12px', padding: '8px' }}
            >
                {isAdded ? '✓ Added to Monitoring' : addMutation.isPending ? 'Adding...' : '+ Add to Monitoring'}
            </button>
        </div>
    );
}

