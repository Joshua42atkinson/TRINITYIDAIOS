import { useState, useEffect } from 'react';

export function useBestiary() {
  const [bestiary, setBestiary] = useState(null);

  const fetchBestiary = async () => {
    try {
      const res = await fetch('/api/bestiary');
      if (!res.ok) return;
      const data = await res.json();
      setBestiary(data);
    } catch { /* silent */ }
  };

  useEffect(() => {
    fetchBestiary();
    const iv = setInterval(fetchBestiary, 8000);
    return () => clearInterval(iv);
  }, []);

  return { bestiary, refetch: fetchBestiary };
}
