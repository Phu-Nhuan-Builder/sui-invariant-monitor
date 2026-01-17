import { InvariantComputation } from '../types/invariant';

interface Props {
    computation: InvariantComputation;
}

export function ComputationDetails({ computation }: Props) {
    const inputs = Object.entries(computation.inputs);

    return (
        <div className="detail-section">
            <h3 className="detail-section__title">Computation Details</h3>

            <div className="computation-formula">
                <strong>Formula:</strong> {computation.formula}
            </div>

            {inputs.length > 0 && (
                <>
                    <h4 style={{ fontSize: '0.75rem', color: 'var(--text-muted)', marginBottom: 'var(--space-sm)' }}>
                        Inputs
                    </h4>
                    <div className="computation-inputs">
                        {inputs.map(([key, value]) => (
                            <div key={key} className="computation-input">
                                <span className="computation-input__key">{key}</span>
                                <span className="computation-input__value">{value}</span>
                            </div>
                        ))}
                    </div>
                </>
            )}

            <div className="computation-result">
                <strong>Result:</strong> {computation.result}
            </div>
        </div>
    );
}
