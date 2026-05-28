<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core'; 

  // --- TYPOVÉ DEFINICE ---
  interface SelectedDay {
    day: number;
    month: number;
    year: number;
  }

  // --- ZÁKLADNÍ STAV ---
  let timeText: string = "";
  let dateText: string = "";
  
  let today: Date = new Date();
  let currentMonth: number = today.getMonth();
  let currentYear: number = today.getFullYear();
  let isYearToggleActive: boolean = false; 
  
  const daysOfWeek: string[] = ['Po', 'Út', 'St', 'Čt', 'Pá', 'So', 'Ne'];
  const monthNames: string[] = [
    "Leden", "Únor", "Březen", "Duben", "Květen", "Červen",
    "Červenec", "Srpen", "Září", "Říjen", "Listopad", "Prosinec"
  ];

  let daysInMonth: (number | null)[] = [];
  
  // --- STAV PRO POZNÁMKY A MODAL ---
  let events: Record<string, string> = {}; 
  let selectedDay: SelectedDay | null = null; 
  let eventText: string = ""; 

  // --- NAČÍTÁNÍ A UKLÁDÁNÍ PŘES RUST ---
  async function loadEvents(): Promise<void> {
    try {
      const data = await invoke<string>('load_event');
      events = JSON.parse(data);
    } catch (error) {
      console.error("Nepodařilo se načíst události:", error);
    }
  }

  async function saveEvent(): Promise<void> {
    if (!selectedDay) return;
    const key = `${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`;

    if (eventText.trim() === "") {
      delete events[key];
    } else {
      events[key] = eventText;
    }

    events = { ...events };

    try {
      await invoke('save_event', { data: JSON.stringify(events) });
    } catch (error) {
      console.error("Nepovedlo se uložit data:", error);
    }

    closeModal();
  }

  // --- NOVINKA: VYMAZÁNÍ UDÁLOSTI ---
  async function deleteEvent(): Promise<void> {
    if (!selectedDay) return;
    const key = `${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`;

    delete events[key]; // Odstraníme záznam
    events = { ...events }; // Vynutíme překreslení UI ve Svelte

    try {
      // Uložíme zbytek dat do Rustu
      await invoke('save_event', { data: JSON.stringify(events) });
    } catch (error) {
      console.error("Nepovedlo se smazat data:", error);
    }

    closeModal();
  }

  // --- OVLÁDÁNÍ MODALU ---
  function handleDayClick(day: number | null): void {
    if (!day) return;
    selectedDay = { day, month: currentMonth, year: currentYear };
    const key = `${currentYear}-${currentMonth}-${day}`;
    eventText = events[key] || ""; 
  }

  function closeModal(): void {
    selectedDay = null;
  }

  // --- LOGIKA KALENDÁŘE ---
  async function calculateCalendar(): Promise<void> {
    let numberOfDays: number;
    try {
      numberOfDays = await invoke<number>('count_days', { 
        monthNo: currentMonth + 1, 
        year: currentYear 
      });
    } catch (error) {
      numberOfDays = new Date(currentYear, currentMonth + 1, 0).getDate();
    }

    const firstDayOfMonth = new Date(currentYear, currentMonth, 1).getDay();
    const startOffset = firstDayOfMonth === 0 ? 6 : firstDayOfMonth - 1;

    let tempDays: (number | null)[] = [];
    for (let i = 0; i < startOffset; i++) {
      tempDays.push(null);
    }
    for (let i = 1; i <= numberOfDays; i++) {
      tempDays.push(i);
    }

    daysInMonth = tempDays;
  }

  function goPrevious(): void {
    if (isYearToggleActive) {
      currentYear--;
    } else {
      currentMonth--;
      if (currentMonth < 0) {
        currentMonth = 11;
        currentYear--;
      }
    }
    calculateCalendar();
  }

  function goNext(): void {
    if (isYearToggleActive) {
      currentYear++;
    } else {
      currentMonth++;
      if (currentMonth > 11) {
        currentMonth = 0;
        currentYear++;
      }
    }
    calculateCalendar();
  }

  function updateTime(): void {
    const now = new Date();
    timeText = now.toLocaleTimeString('cs-CZ', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    dateText = now.toLocaleDateString('cs-CZ', { day: 'numeric', month: 'long' });
  }

  onMount(() => {
    updateTime();
    calculateCalendar();
    loadEvents();
    const interval = setInterval(updateTime, 1000);
    return () => clearInterval(interval);
  });
</script>

<main class="app-container">
  <div class="main-widget">
    <div class="widget-layout">
      
      <div class="calendar-header">
        <button class="nav-btn" on:click={goPrevious}>&#8592;</button>
        <div class="title-toggle">
          <span class="month-part" class:active={!isYearToggleActive} on:click={() => isYearToggleActive = false}>
            {monthNames[currentMonth]}
          </span>
          <span class="year-part" class:active={isYearToggleActive} on:click={() => isYearToggleActive = true}>
            {currentYear}
          </span>
        </div>
        <button class="nav-btn" on:click={goNext}>&#8594;</button>
      </div>

      <div class="status-clock">
        <span class="time">{timeText}</span>
        <span class="date-small">{dateText}</span>
      </div>

      <div class="calendar-grid">
        {#each daysOfWeek as dayName}
          <div class="weekday-label">{dayName}</div>
        {/each}

        {#each daysInMonth as day}
          {@const key = day ? `${currentYear}-${currentMonth}-${day}` : ""}
          <div 
            class="day-cell" 
            class:today={day === today.getDate() && currentMonth === today.getMonth() && currentYear === today.getFullYear()}
            on:click={() => handleDayClick(day)}
          >
            {day || ""}
            {#if day && events[key]}
              <div class="event-dot"></div>
            {/if}
          </div>
        {/each}
      </div>

    </div>

    {#if selectedDay}
      <div class="modal-overlay" on:click={closeModal}>
        <div class="modal-content" on:click|stopPropagation>
          <h3>Poznámka: {selectedDay.day}. {monthNames[selectedDay.month]} {selectedDay.year}</h3>
          
          <textarea bind:value={eventText} placeholder="Sem zapiš svou událost..."></textarea>
          
          <div class="modal-actions">
            {#if events[`${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`]}
              <button class="btn-delete" on:click={deleteEvent}>Vymazat</button>
            {/if}
            <button class="btn-cancel" on:click={closeModal}>Zrušit</button>
            <button class="btn-save" on:click={saveEvent}>Uložit</button>
          </div>
        </div>
      </div>
    {/if}

  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: #0f0f0f url('https://images.unsplash.com/photo-1635776062127-d379bfcba9f8?q=80&w=2000&auto=format&fit=crop') no-repeat center center fixed;
    background-size: cover;
    color: white;
    font-family: 'Segoe UI Variable Display', 'Segoe UI', sans-serif;
    overflow: hidden;
  }

  .app-container {
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    box-sizing: border-box;
    backdrop-filter: brightness(0.65);
  }

  .main-widget {
    position: relative; 
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(25px);
    -webkit-backdrop-filter: blur(25px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    padding: 40px;
    width: 700px;
    box-shadow: 0 30px 60px rgba(0,0,0,0.5);
    box-sizing: border-box;
  }

  .widget-layout {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 20px;
    align-items: center;
  }

  .calendar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding-right: 15px;
    box-sizing: border-box;
  }

  .title-toggle {
    display: flex;
    gap: 12px;
    justify-content: center;
    align-items: baseline;
    width: 280px; 
  }

  .title-toggle span {
    cursor: pointer;
    transition: all 0.25s ease;
    user-select: none; 
  }

  .title-toggle span:hover { filter: brightness(1.3); }

  .month-part { font-size: 2.6rem; font-weight: 500; color: #0078d4; opacity: 0.35; }
  .month-part.active { opacity: 1; }

  .year-part { font-size: 2.3rem; font-weight: 300; color: white; opacity: 0.25; }
  .year-part.active { opacity: 1; font-weight: 500; }

  .nav-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    border-radius: 50%;
    width: 42px;
    height: 42px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.2rem;
    flex-shrink: 0; 
  }
  .nav-btn:hover { background: rgba(255, 255, 255, 0.15); border-color: rgba(255, 255, 255, 0.3); }

  .status-clock {
    text-align: center;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 12px 25px;
    border-radius: 16px;
    min-width: 160px;
  }

  .time { display: block; font-size: 2.2rem; font-weight: 500; line-height: 1.1; font-variant-numeric: tabular-nums; }
  .date-small { display: block; font-size: 0.85rem; opacity: 0.6; margin-top: 4px; }

  .calendar-grid {
    grid-column: span 2;
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 12px;
    text-align: center;
    margin-top: 20px;
  }

  .weekday-label { font-weight: 600; font-size: 0.95rem; color: #0078d4; padding-bottom: 5px; text-transform: uppercase; }

  .day-cell {
    position: relative; 
    width: 46px;      
    height: 46px;     
    margin: 0 auto;   
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    font-size: 1.25rem;
    transition: background 0.15s;
    user-select: none;
  }

  .day-cell:hover:not(:empty) {
    background: rgba(255, 255, 255, 0.1);
    cursor: pointer;
  }

  .today {
    background: #0078d4 !important;
    font-weight: bold;
    color: white;
    box-shadow: 0 0 20px rgba(0, 120, 212, 0.6);
  }

  .event-dot {
    position: absolute;
    bottom: 5px;
    width: 5px;
    height: 5px;
    background: white;
    border-radius: 50%;
    opacity: 0.7;
  }
  
  .today .event-dot { background: rgba(255, 255, 255, 0.9); }

  /* --- DESIGN VYSKAKOVACÍHO OKNA --- */
  .modal-overlay {
    position: absolute;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(8px);
    border-radius: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-content {
    background: rgba(40, 40, 40, 0.95);
    border: 1px solid rgba(255, 255, 255, 0.15);
    padding: 25px;
    border-radius: 16px;
    width: 320px;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
    animation: popIn 0.2s ease-out;
  }

  @keyframes popIn {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }

  .modal-content h3 {
    margin: 0 0 15px 0;
    font-weight: 500;
    color: #0078d4;
  }

  .modal-content textarea {
    width: 100%;
    height: 120px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    font-family: inherit;
    font-size: 1rem;
    padding: 12px;
    border-radius: 8px;
    resize: none;
    box-sizing: border-box;
  }

  .modal-content textarea:focus {
    outline: none;
    border-color: #0078d4;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end; /* Zarovná tlačítka primárně doprava */
    gap: 10px;
    margin-top: 15px;
  }

  .modal-actions button {
    padding: 8px 16px;
    border-radius: 6px;
    border: none;
    cursor: pointer;
    font-weight: 500;
    transition: 0.2s;
  }

  /* --- STYL TLAČÍTKA VYMAZAT --- */
  .btn-delete {
    margin-right: auto; /* Tento magický CSS řádek odtlačí Zrušit a Uložit doprava, zatímco on zůstane vlevo */
    background: rgba(220, 53, 69, 0.15);
    color: #ff6b6b;
  }

  .btn-delete:hover { 
    background: rgba(220, 53, 69, 0.8); 
    color: white;
  }

  .btn-cancel {
    background: rgba(255, 255, 255, 0.1);
    color: white;
  }

  .btn-cancel:hover { background: rgba(255, 255, 255, 0.2); }

  .btn-save {
    background: #0078d4;
    color: white;
  }

  .btn-save:hover { background: #005a9e; }
</style>