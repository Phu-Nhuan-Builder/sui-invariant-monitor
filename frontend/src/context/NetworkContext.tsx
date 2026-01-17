import { createContext, useContext, useState, ReactNode } from 'react';

export type Network = 'mainnet' | 'testnet';

interface NetworkContextType {
    network: Network;
    setNetwork: (network: Network) => void;
    rpcUrl: string;
}

const NETWORK_RPC_URLS: Record<Network, string> = {
    mainnet: 'https://fullnode.mainnet.sui.io:443',
    testnet: 'https://fullnode.testnet.sui.io:443',
};

const NetworkContext = createContext<NetworkContextType | undefined>(undefined);

export function NetworkProvider({ children }: { children: ReactNode }) {
    const [network, setNetwork] = useState<Network>('mainnet');

    const value = {
        network,
        setNetwork,
        rpcUrl: NETWORK_RPC_URLS[network],
    };

    return (
        <NetworkContext.Provider value={value}>
            {children}
        </NetworkContext.Provider>
    );
}

export function useNetwork() {
    const context = useContext(NetworkContext);
    if (!context) {
        throw new Error('useNetwork must be used within a NetworkProvider');
    }
    return context;
}
