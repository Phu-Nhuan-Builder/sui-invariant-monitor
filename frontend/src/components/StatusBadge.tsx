import { InvariantStatus } from '../types/invariant';

interface Props {
    status: InvariantStatus;
    showLabel?: boolean;
}

export function StatusBadge({ status, showLabel = true }: Props) {
    const statusConfig = {
        Ok: { label: 'OK', className: 'status-badge--ok' },
        Violated: { label: 'VIOLATED', className: 'status-badge--violated' },
        Error: { label: 'ERROR', className: 'status-badge--error' },
    };

    const config = statusConfig[status];

    return (
        <span className={`status-badge ${config.className}`}>
            <span className={`status-indicator status-indicator--${status.toLowerCase()}`} />
            {showLabel && config.label}
        </span>
    );
}
