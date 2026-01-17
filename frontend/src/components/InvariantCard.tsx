import { Link } from 'react-router-dom';
import { InvariantResult } from '../types/invariant';
import { StatusBadge } from './StatusBadge';

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
    return (
        <Link to={`/invariant/${invariant.id}`} className="invariant-card">
            <div className="invariant-card__header">
                <div>
                    <div className="invariant-card__id">{invariant.id}</div>
                    <div className="invariant-card__name">{invariant.name}</div>
                </div>
                <StatusBadge status={invariant.status} />
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
