import { useState, useRef, useEffect } from 'react';
import { useNetwork, Network } from '../context/NetworkContext';

export function NetworkSwitcher() {
    const { network, setNetwork } = useNetwork();
    const [isOpen, setIsOpen] = useState(false);
    const dropdownRef = useRef<HTMLDivElement>(null);

    // Close dropdown when clicking outside
    useEffect(() => {
        function handleClickOutside(event: MouseEvent) {
            if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
                setIsOpen(false);
            }
        }
        document.addEventListener('mousedown', handleClickOutside);
        return () => document.removeEventListener('mousedown', handleClickOutside);
    }, []);

    const handleSelect = (net: Network) => {
        setNetwork(net);
        setIsOpen(false);
    };

    return (
        <div className="network-switcher" ref={dropdownRef}>
            <button
                className={`network-button ${isOpen ? 'open' : ''}`}
                onClick={() => setIsOpen(!isOpen)}
            >
                <span className={`network-dot ${network}`} />
                <span>{network === 'mainnet' ? 'Mainnet' : 'Testnet'}</span>
                <span className="chevron">▼</span>
            </button>

            {isOpen && (
                <div className="network-dropdown">
                    <div
                        className={`network-option ${network === 'mainnet' ? 'active' : ''}`}
                        onClick={() => handleSelect('mainnet')}
                    >
                        <span className="network-dot mainnet" />
                        Mainnet
                    </div>
                    <div
                        className={`network-option ${network === 'testnet' ? 'active' : ''}`}
                        onClick={() => handleSelect('testnet')}
                    >
                        <span className="network-dot testnet" />
                        Testnet
                    </div>
                </div>
            )}
        </div>
    );
}
