<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  // --- LOGIKA HODIN ---
  let cas = "";
  let datumText = "";

  // --- LOGIKA KALENDÁŘE ---
  let dnes = new Date();
  let aktualniMesic = dnes.getMonth();
  let aktualniRok = dnes.getFullYear();

  const dnyVTyzdnu = ["Po", "Út", "St", "Čt", "Pá", "So", "Ne"];
  const nazvyMesicu = [
    "Leden",
    "Únor",
    "Březen",
    "Duben",
    "Květen",
    "Červen",
    "Červenec",
    "Srpen",
    "Září",
    "Říjen",
    "Listopad",
    "Prosinec",
  ];

  /** @type {(number|null)[]} */
  let dnyVMesici = [];

  async function vypocitejKalendar() {
    // Voláme tvůj Rust backend (množné číslo převedeno na camelCase)
    const pocetDni = await invoke("spocitej_dny_v_mesici", {
      mesicCislo: aktualniMesic + 1,
      rok: aktualniRok,
    });

    const prvniDenMesice = new Date(aktualniRok, aktualniMesic, 1).getDay();
    const startOffset = prvniDenMesice === 0 ? 6 : prvniDenMesice - 1;

    let docasneDny = [];
    // Prázdná místa před začátkem měsíce
    for (let i = 0; i < startOffset; i++) {
      docasneDny.push(null);
    }
    // Dny v měsíci (číslo vrácené z Rustu)
    for (let i = 1; i <= pocetDni; i++) {
      docasneDny.push(i);
    }

    // Přiřazení celého pole naráz pro vynucení reaktivity ve Svelte
    dnyVMesici = docasneDny;
  }

  function aktualizujCas() {
    const ted = new Date();
    // Přidány sekundy a fixní šířka čísel (tabular-nums), aby hodiny neskákaly
    cas = ted.toLocaleTimeString("cs-CZ", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
    datumText = ted.toLocaleDateString("cs-CZ", {
      day: "numeric",
      month: "long",
    });
  }

  onMount(() => {
    aktualizujCas();
    vypocitejKalendar();
    const interval = setInterval(aktualizujCas, 1000);
    return () => clearInterval(interval);
  });
</script>

<main class="app-container">
  <div class="main-widget">
    <div class="widget-layout">
      <div class="calendar-header">
        <h2>
          {nazvyMesicu[aktualniMesic]} <span class="year">{aktualniRok}</span>
        </h2>
      </div>

      <div class="status-clock">
        <span class="time">{cas}</span>
        <span class="date-small">{datumText}</span>
      </div>

      <div class="calendar-grid">
        {#each dnyVTyzdnu as den}
          <div class="weekday-label">{den}</div>
        {/each}

        {#each dnyVMesici as den}
          <div
            class="day-cell"
            class:today={den === dnes.getDate() &&
              aktualniMesic === dnes.getMonth()}
          >
            {den || ""}
          </div>
        {/each}
      </div>
    </div>
  </div>
</main>

<style>
  /* ZÁKLADNÍ NASTAVENÍ */
  :global(body) {
    margin: 0;
    padding: 0;
    background: #0f0f0f
      url("https://images.unsplash.com/photo-1635776062127-d379bfcba9f8?q=80&w=2000&auto=format&fit=crop")
      no-repeat center center fixed;
    background-size: cover;
    color: white;
    font-family: "Segoe UI Variable Display", "Segoe UI", sans-serif;
    overflow: hidden;
  }

  .app-container {
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    box-sizing: border-box;
    backdrop-filter: brightness(0.65); /* Ztmavíme pozadí */
  }

  /* HLAVNÍ GLASSMORPHISM PANEL */
  .main-widget {
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(25px);
    -webkit-backdrop-filter: blur(25px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    padding: 40px;
    width: 700px; /* Šířka upravená tak, aby měla neděle dostatek místa */
    box-shadow: 0 30px 60px rgba(0, 0, 0, 0.5);
    box-sizing: border-box;
  }

  /* ROZVRŽENÍ VNITŘKU */
  .widget-layout {
    display: grid;
    grid-template-columns: 1fr auto; /* Hlavička vlevo, čas vpravo */
    gap: 20px;
    align-items: center;
  }

  .calendar-header h2 {
    margin: 0;
    font-weight: 500;
    font-size: 2.6rem;
    color: #0078d4; /* Windows modrá */
  }

  .calendar-header h2 .year {
    color: white;
    opacity: 0.4;
    font-weight: 300;
    margin-left: 8px;
  }

  /* ČAS VPRAVO NAHOŘE */
  .status-clock {
    text-align: center;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 12px 25px;
    border-radius: 16px;
    min-width: 160px;
  }

  .time {
    display: block;
    font-size: 2.2rem;
    font-weight: 500;
    line-height: 1.1;
    font-variant-numeric: tabular-nums; /* Čísla neposkakují při změně vteřin */
  }

  .date-small {
    display: block;
    font-size: 0.85rem;
    opacity: 0.6;
    margin-top: 4px;
  }

  /* MŘÍŽKA KALENDÁŘE */
  .calendar-grid {
    grid-column: span 2; /* Roztáhne kalendář pod měsíc i pod hodiny */
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 12px;
    text-align: center;
    margin-top: 20px;
  }

  .weekday-label {
    font-weight: 600;
    font-size: 0.95rem;
    color: #0078d4;
    padding-bottom: 5px;
    text-transform: uppercase;
  }

  .day-cell {
    height: 52px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    font-size: 1.25rem;
    transition: background 0.15s;
  }

  .day-cell:hover:not(:empty) {
    background: rgba(255, 255, 255, 0.1);
    cursor: pointer;
  }

  /* DNEŠNÍ DEN */
  .today {
    background: #0078d4 !important;
    font-weight: bold;
    color: white;
    box-shadow: 0 0 20px rgba(0, 120, 212, 0.6);
  }
</style>
