import { Outlet } from 'react-router-dom';
import { useMonitorStatus } from '../hooks/useInvariants';
import { NetworkSwitcher } from './NetworkSwitcher';

function formatDateTime(isoString: string | null): string {
    if (!isoString) return '—';
    const date = new Date(isoString);
    return date.toLocaleTimeString('en-US', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
    });
}

export function Layout() {
    const { data: status } = useMonitorStatus();

    return (
        <div className="layout">
            <header className="header">
                <div className="header-content">
                    <div className="header-title">
                        <span className="logo">🛡️</span>
                        <h1>Sui Invariant Monitor</h1>
                    </div>

                    <div className="header-right">
                        <div className="header-status">
                            <span>Last check: {formatDateTime(status?.last_check ?? null)}</span>
                            {status && (
                                <span className={`status-badge ${status.all_ok ? 'status-badge--ok' : 'status-badge--violated'}`}>
                                    <span className={`status-indicator ${status.all_ok ? 'status-indicator--ok' : 'status-indicator--violated'}`} />
                                    {status.all_ok ? 'All OK' : `${status.violations} Violation${status.violations !== 1 ? 's' : ''}`}
                                </span>
                            )}
                        </div>
                        <NetworkSwitcher />
                    </div>
                </div>
            </header>

            <main className="main">
                <Outlet />
            </main>

            <footer className="footer">
                <div className="footer-content">
                    <img
                        src="/phunhuanbuilder-logo-zoom.png"
                        alt="Phú Nhuận Builder"
                        className="footer-logo"
                    />
                    <p className="footer-text">
                        © 2026 Phú Nhuận Builder. Built for First Movers Sprint 2026
                    </p>
                </div>
            </footer>
        </div>
    );
}
