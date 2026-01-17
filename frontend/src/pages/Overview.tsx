import { useInvariants, useMonitorStatus } from '../hooks/useInvariants';
import { InvariantCard } from '../components/InvariantCard';
import { AnalyzeContractForm } from '../components/AnalyzeContractForm';

export function Overview() {
    const { data: invariants, isLoading, error } = useInvariants();
    const { data: status } = useMonitorStatus();

    return (
        <div className="overview-page">
            {/* AI Analysis Section */}
            <AnalyzeContractForm />

            {status && status.monitored_objects.length > 0 && (
                <>
                    <div className="monitored-objects">
                        <h4>Monitoring {status.monitored_objects.length} object(s)</h4>
                        <div className="object-list">
                            {status.monitored_objects.map((id) => (
                                <span key={id} className="object-tag" title={id}>
                                    {id.slice(0, 8)}...{id.slice(-6)}
                                </span>
                            ))}
                        </div>
                    </div>
                </>
            )}

            <div className="section-divider" />

            <h3 className="section-title">Invariant Status</h3>

            {isLoading && (
                <div className="loading">
                    <div className="loading-spinner" />
                    Loading invariants...
                </div>
            )}

            {error && (
                <div className="error">
                    Failed to load invariants: {error.message}
                </div>
            )}

            {!isLoading && !error && invariants && invariants.length === 0 && (
                <div className="empty-state">
                    <p>No invariants to display yet.</p>
                    <p className="empty-hint">Use AI analysis above to generate invariants.</p>
                </div>
            )}

            {invariants && invariants.length > 0 && (
                <div className="invariant-grid">
                    {invariants.map((inv) => (
                        <InvariantCard key={inv.id} invariant={inv} />
                    ))}
                </div>
            )}
        </div>
    );
}
