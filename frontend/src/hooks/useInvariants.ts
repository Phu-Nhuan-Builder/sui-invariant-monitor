import { useQuery } from '@tanstack/react-query';
import { fetchInvariants, fetchInvariant, fetchStatus } from '../api/client';

const POLL_INTERVAL = 10_000; // 10 seconds

export function useInvariants() {
    return useQuery({
        queryKey: ['invariants'],
        queryFn: fetchInvariants,
        refetchInterval: POLL_INTERVAL,
    });
}

export function useInvariant(id: string) {
    return useQuery({
        queryKey: ['invariant', id],
        queryFn: () => fetchInvariant(id),
        refetchInterval: POLL_INTERVAL,
    });
}

export function useMonitorStatus() {
    return useQuery({
        queryKey: ['status'],
        queryFn: fetchStatus,
        refetchInterval: POLL_INTERVAL,
    });
}
