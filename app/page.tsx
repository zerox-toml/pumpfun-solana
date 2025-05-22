'use client';

import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useState } from 'react';

export default function Home() {
  const { publicKey } = useWallet();
  const [amount, setAmount] = useState('');

  return (
    <div className="container mx-auto px-4 py-8">
      <header className="flex justify-between items-center mb-12">
        <h1 className="text-4xl font-bold bg-gradient-to-r from-purple-400 to-pink-600 bg-clip-text text-transparent">
          PumpFun
        </h1>
        <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
      </header>

      <div className="max-w-2xl mx-auto">
        <div className="bg-gray-800 rounded-lg p-6 shadow-xl">
          <h2 className="text-2xl font-semibold mb-4">Bonding Curve</h2>
          
          {publicKey ? (
            <div className="space-y-4">
              <div className="flex flex-col space-y-2">
                <label className="text-sm text-gray-300">Amount (SOL)</label>
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  className="bg-gray-700 rounded px-4 py-2 focus:outline-none focus:ring-2 focus:ring-purple-500"
                  placeholder="Enter amount"
                />
              </div>
              
              <button
                className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded transition-colors"
                onClick={() => {
                  // TODO: Implement buy functionality
                  console.log('Buy clicked');
                }}
              >
                Buy Tokens
              </button>
            </div>
          ) : (
            <div className="text-center py-8">
              <p className="text-gray-400 mb-4">Connect your wallet to start trading</p>
            </div>
          )}
        </div>

        <div className="mt-8 grid grid-cols-2 gap-4">
          <div className="bg-gray-800 rounded-lg p-4">
            <h3 className="text-lg font-semibold mb-2">Current Price</h3>
            <p className="text-2xl text-purple-400">0.1 SOL</p>
          </div>
          <div className="bg-gray-800 rounded-lg p-4">
            <h3 className="text-lg font-semibold mb-2">Total Supply</h3>
            <p className="text-2xl text-purple-400">1,000,000</p>
          </div>
        </div>
      </div>
    </div>
  );
} 