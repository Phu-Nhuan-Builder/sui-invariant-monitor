import { Link, useParams } from 'react-router-dom';
import { useInvariant } from '../hooks/useInvariants';
import { StatusBadge } from '../components/StatusBadge';
import { ComputationDetails } from '../components/ComputationDetails';

function formatDateTime(isoString: string): string {
    const date = new Date(isoString);
    return date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
    });
}

export function InvariantDetail() {
    const { id } = useParams<{ id: string }>();
    const { data: invariant, isLoading, error } = useInvariant(id || '');

    if (isLoading) {
        return (
            <div className="loading">
                <div className="loading-spinner" />
                Loading invariant...
            </div>
        );
    }

    if (error || !invariant) {
        return (
            <div className="error">
                Invariant not found: {id}
            </div>
        );
    }

    return (
        <div className="detail-page">
            <Link to="/" className="back-link">
                ← Back to Overview
            </Link>

            <div className="detail-header">
                <div className="detail-header__title">
                    <StatusBadge status={invariant.status} />
                    <h2 className="detail-header__name">
                        {invariant.id}: {invariant.name}
                    </h2>
                </div>
                <p className="detail-header__description">{invariant.description}</p>
            </div>

            <div className="metadata">
                <div className="metadata__item">
                    <span>🕐</span>
                    <span>Last Evaluated: {formatDateTime(invariant.evaluated_at)}</span>
                </div>
            </div>

            {invariant.violation_reason && (
                <div className="detail-section">
                    <h3 className="detail-section__title">Violation Reason</h3>
                    <div className="violation-reason">
                        {invariant.violation_reason}
                    </div>
                </div>
            )}

            <ComputationDetails computation={invariant.computation} />
        </div>
    );
}
