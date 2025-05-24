import { TimeRange } from "@/stores/filters";

// Renvoie une liste de dates/horaires adaptées à chaque plage
export function generateTimeline(timeRange: TimeRange): string[] {
  const now = new Date();
  const result: string[] = [];

  function formatDate(date: Date): string {
    return date.toISOString().slice(0, 10); // "YYYY-MM-DD"
  }

  function formatHour(date: Date): string {
    return date.toISOString().slice(0, 13) + ":00"; // "YYYY-MM-DDTHH:00"
  }

  switch (timeRange) {
    case "1d": {
      // 9h à 17h (heure pleine)
      const base = new Date(now);
      base.setMinutes(0, 0, 0);
      for (let h = 9; h <= 17; h++) {
        const point = new Date(base);
        point.setHours(h);
        result.push(formatHour(point));
      }
      break;
    }

    case "7d": {
      for (let i = 6; i >= 0; i--) {
        const d = new Date(now);
        d.setDate(now.getDate() - i);
        result.push(formatDate(d));
      }
      break;
    }

    case "1m": {
      for (let i = 29; i >= 0; i -= 3) {
        const d = new Date(now);
        d.setDate(now.getDate() - i);
        result.push(formatDate(d));
      }
      break;
    }

    case "1y": {
      for (let i = 11; i >= 0; i--) {
        const d = new Date(now);
        d.setMonth(now.getMonth() - i);
        result.push(formatDate(d));
      }
      break;
    }

    case "ytd": {
      const start = new Date(now.getFullYear(), 0, 1);
      while (start <= now) {
        result.push(formatDate(new Date(start)));
        start.setMonth(start.getMonth() + 1, 1);
      }
      break;
    }

    case "all": {
      // Laisse le widget gérer "all" en fonction des transactions
      break;
    }
  }

  return result;
}
