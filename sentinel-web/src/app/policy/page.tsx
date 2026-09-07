import { Lock } from 'lucide-react';

export default function PolicyPage() {
  return (
    <div className="max-w-3xl mx-auto animate-in fade-in duration-500">
      <header className="mb-8">
        <h1 className="text-3xl font-bold text-slate-100">Trading Policy</h1>
        <p className="text-slate-500 mt-2">Active Intent Contract: IC-001</p>
      </header>

      <div className="bg-slate-900 border border-slate-800 rounded-xl p-8">
        <div className="flex items-center space-x-2 text-amber-500 mb-8 p-4 bg-amber-500/10 rounded-lg border border-amber-500/20">
          <Lock size={18} />
          <span className="text-sm font-medium">Policy is locked. AI Agents cannot mutate their own intent constraints.</span>
        </div>

        <div className="space-y-8">
          <div>
            <h3 className="text-lg font-medium text-slate-200 mb-4">Allowed Assets</h3>
            <div className="flex space-x-3">
              <AssetBadge symbol="BTC" active />
              <AssetBadge symbol="ETH" active />
              <AssetBadge symbol="BNB" active={false} />
              <AssetBadge symbol="SOL" active={false} />
            </div>
          </div>

          <div className="grid grid-cols-2 gap-6">
            <PolicyInput label="Maximum Position Size" value="10%" />
            <PolicyInput label="Maximum Trade Size" value="2%" />
            <PolicyInput label="Maximum Daily Loss" value="3%" />
            <PolicyInput label="Maximum Leverage" value="2x" />
          </div>

          <div className="pt-6 border-t border-slate-800 flex justify-between items-center">
            <div>
              <div className="text-slate-200 font-medium">Require External Evidence</div>
              <div className="text-sm text-slate-500">Must verify market conditions before execution</div>
            </div>
            <div className="w-12 h-6 bg-emerald-500 rounded-full relative">
              <div className="absolute right-1 top-1 w-4 h-4 bg-white rounded-full"></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function AssetBadge({ symbol, active }: { symbol: string, active: boolean }) {
  return (
    <div className={`px-4 py-2 rounded-lg font-bold text-sm border ${
      active 
        ? "bg-emerald-500/20 text-emerald-400 border-emerald-500/30" 
        : "bg-slate-800 text-slate-500 border-slate-700"
    }`}>
      {symbol}
    </div>
  );
}

function PolicyInput({ label, value }: { label: string, value: string }) {
  return (
    <div>
      <label className="block text-sm text-slate-500 mb-2">{label}</label>
      <div className="bg-slate-950 border border-slate-800 text-slate-300 font-mono px-4 py-3 rounded-lg w-full">
        {value}
      </div>
    </div>
  );
}