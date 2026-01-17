import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { NetworkProvider } from './context/NetworkContext';
import { Layout } from './components/Layout';
import { Overview } from './pages/Overview';
import { InvariantDetail } from './pages/InvariantDetail';

const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            staleTime: 5000,
            retry: 2,
        },
    },
});

function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <NetworkProvider>
                <BrowserRouter>
                    <Routes>
                        <Route path="/" element={<Layout />}>
                            <Route index element={<Overview />} />
                            <Route path="invariant/:id" element={<InvariantDetail />} />
                        </Route>
                    </Routes>
                </BrowserRouter>
            </NetworkProvider>
        </QueryClientProvider>
    );
}

export default App;
