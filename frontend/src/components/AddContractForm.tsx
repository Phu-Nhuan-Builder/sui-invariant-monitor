import { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { addMonitoredObject } from '../api/client';

export function AddContractForm() {
    const [objectId, setObjectId] = useState('');
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const queryClient = useQueryClient();

    const mutation = useMutation({
        mutationFn: addMonitoredObject,
        onSuccess: (data) => {
            if (data.success) {
                setSuccess(data.message);
                setError(null);
                setObjectId('');
                queryClient.invalidateQueries({ queryKey: ['invariants'] });
                queryClient.invalidateQueries({ queryKey: ['status'] });
            } else {
                setError(data.message);
                setSuccess(null);
            }
        },
        onError: (err: Error) => {
            setError(err.message);
            setSuccess(null);
        },
    });

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setSuccess(null);

        if (!objectId.trim()) {
            setError('Please enter an Object ID');
            return;
        }

        mutation.mutate({ object_id: objectId.trim() });
    };

    return (
        <div className="form-section add-contract-form">
            <h3 className="form-title">Monitor Object</h3>
            <p className="form-description">
                Enter a Sui Object ID to start monitoring its state
            </p>

            <form onSubmit={handleSubmit} className="form-content">
                <div className="input-group">
                    <input
                        type="text"
                        value={objectId}
                        onChange={(e) => setObjectId(e.target.value)}
                        placeholder="0x... (64 hex characters)"
                        className="text-input"
                        disabled={mutation.isPending}
                    />
                    <button
                        type="submit"
                        className="btn btn-secondary"
                        disabled={mutation.isPending}
                    >
                        {mutation.isPending ? 'Adding...' : 'Monitor'}
                    </button>
                </div>

                {error && <div className="form-message error">✕ {error}</div>}
                {success && <div className="form-message success">✓ {success}</div>}
            </form>
        </div>
    );
}
