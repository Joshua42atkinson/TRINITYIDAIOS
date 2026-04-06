import { useState, useEffect } from 'react';

export function usePearl() {
  const [pearl, setPearl] = useState(null);

  const fetchPearl = async () => {
    try {
      const res = await fetch('/api/pearl');
      if (!res.ok) return;
      const data = await res.json();
      if (!data.error) setPearl(data);
    } catch { /* silent */ }
  };

  useEffect(() => {
    fetchPearl();
    const iv = setInterval(fetchPearl, 5000);
    return () => clearInterval(iv);
  }, []);

  const updatePearl = async (updates) => {
    try {
      const res = await fetch('/api/pearl/refine', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates),
      });
      if (res.ok) {
        await fetchPearl();
      }
    } catch (err) {
      console.error('PEARL update failed:', err);
    }
  };

  return { pearl, refetch: fetchPearl, updatePearl };
}
