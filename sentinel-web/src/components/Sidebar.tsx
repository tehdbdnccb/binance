import Link from 'next/link';
import { Shield, Activity, FileText, Database } from 'lucide-react';

export function Sidebar() {
  return (
    <div className="w-64 bg-slate-950 border-r border-slate-800 h-screen flex flex-col">
      <div className="h-16 flex items-center px-6 border-b border-slate-800">
        <Shield className="text-emerald-400 mr-3" />
        <span className="text-lg font-bold tracking-widest text-slate-100">SENTINEL</span>
      </div>
      <nav className="flex-1 py-6 flex flex-col space-y-2 px-4">
        <NavLink href="/" icon={<Activity size={18} />} label="Dashboard" />
        <NavLink href="/decisions" icon={<Database size={18} />} label="Decision Ledger" />
        <NavLink href="/policy" icon={<FileText size={18} />} label="Trading Policy" />
      </nav>
      <div className="p-4 border-t border-slate-800">
        <div className="flex items-center space-x-3 bg-slate-900 rounded-lg p-3 border border-slate-800">
          <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></div>
          <div className="text-sm">
            <p className="text-slate-300 font-medium">Binance OS</p>
            <p className="text-slate-500 text-xs">Connected</p>
          </div>
        </div>
      </div>
    </div>
  );
}

function NavLink({ href, icon, label }: { href: string, icon: React.ReactNode, label: string }) {
  return (
    <Link href={href} className="flex items-center space-x-3 px-4 py-3 text-slate-400 hover:text-slate-100 hover:bg-slate-900 rounded-lg transition-colors">
      {icon}
      <span className="font-medium text-sm">{label}</span>
    </Link>
  );
}