import { Link } from 'react-router-dom';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { InvariantResult } from '../types/invariant';
import { StatusBadge } from './StatusBadge';
import { removeInvariant } from '../api/client';

interface Props {
    invariant: InvariantResult;
}

function formatTime(isoString: string): string {
    const date = new Date(isoString);
    return date.toLocaleTimeString('en-US', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
    });
}

export function InvariantCard({ invariant }: Props) {
    const queryClient = useQueryClient();

    const removeMutation = useMutation({
        mutationFn: () => removeInvariant({ invariant_id: invariant.id }),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['invariants'] });
            queryClient.invalidateQueries({ queryKey: ['status'] });
        },
    });

    const handleRemove = (e: React.MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();

        if (confirm(`Remove invariant "${invariant.name}" from monitoring?`)) {
            removeMutation.mutate();
        }
    };

    return (
        <Link to={`/invariant/${invariant.id}`} className="invariant-card">
            <div className="invariant-card__header">
                <div>
                    <div className="invariant-card__id">{invariant.id}</div>
                    <div className="invariant-card__name">{invariant.name}</div>
                </div>
                <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                    <StatusBadge status={invariant.status} />
                    <button
                        onClick={handleRemove}
                        disabled={removeMutation.isPending}
                        className="btn-icon-remove"
                        title="Remove from monitoring"
                    >
                        {removeMutation.isPending ? '...' : '−'}
                    </button>
                </div>
            </div>

            <div className="invariant-card__description">
                {invariant.description}
            </div>

            <div className="invariant-card__footer">
                <span>Last: {formatTime(invariant.evaluated_at)}</span>
                <span className="invariant-card__arrow">→</span>
            </div>
        </Link>
    );
}
