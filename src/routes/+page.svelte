<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { supabase } from "$lib/supabase";

  // --- TYPOVÉ DEFINICE ---
  interface SelectedDay {
    day: number;
    month: number;
    year: number;
  }
  interface AppSettings {
    language: "cs" | "en";
    clockStyle: "digital" | "minimal" | "analog";
    backgroundUrl: string;
    themeColor: "blue" | "red" | "gray" | "purple" | "green";
  }
  type SidebarView = "menu" | "settings" | "credits";

  // --- AUTHENTIKACE STAV ---
  let session: any = null;
  let authEmail = "";
  let authPassword = "";
  let isLoginMode = true;
  let authError = "";
  let isAuthLoading = false;

  // --- REAKTIVNÍ STAV APLIKACE ---
  let timeText: string = "";
  let dateText: string = "";
  let hourAngle: number = 0;
  let minuteAngle: number = 0;
  let secondAngle: number = 0;

  let today: Date = new Date();
  let currentMonth: number = today.getMonth();
  let currentYear: number = today.getFullYear();
  let isYearToggleActive: boolean = false;

  let settings: AppSettings = {
    language: "cs",
    clockStyle: "digital",
    backgroundUrl:
      "https://images.unsplash.com/photo-1635776062127-d379bfcba9f8?q=80&w=2000&auto=format&fit=crop",
    themeColor: "blue",
  };

  const daysOfWeekCS = ["Po", "Út", "St", "Čt", "Pá", "So", "Ne"];
  const daysOfWeekEN = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
  const monthNamesCS = [
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
  const monthNamesEN = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  let daysInMonth: (number | null)[] = [];
  let events: Record<string, string> = {};
  let selectedDay: SelectedDay | null = null;
  let eventText: string = "";
  let isMenuOpen: boolean = false;
  let currentSidebarView: SidebarView = "menu";

  // --- AUTH LOGIKA ---
  async function handleAuth() {
    isAuthLoading = true;
    authError = "";

    if (isLoginMode) {
      const { error } = await supabase.auth.signInWithPassword({
        email: authEmail,
        password: authPassword,
      });
      if (error) authError = error.message;
    } else {
      const { error } = await supabase.auth.signUp({
        email: authEmail,
        password: authPassword,
      });
      if (error) authError = error.message;
    }

    isAuthLoading = false;
  }

  async function handleLogout() {
    await supabase.auth.signOut();
    events = {};
    isMenuOpen = false;
  }

  // --- CLOUD NAČÍTÁNÍ A UKLÁDÁNÍ PŘES SUPABASE ---
  async function loadSettings(): Promise<void> {
    if (!session) return;
    try {
      const { data, error } = await supabase
        .from("user_settings")
        .select("*")
        .eq("user_id", session.user.id)
        .maybeSingle();
      if (data) {
        settings.language = data.language || "cs";
        settings.clockStyle = data.clock_style || "digital";
        settings.themeColor = data.theme_color || "blue";
        settings.backgroundUrl = data.background_url || settings.backgroundUrl;
      }
    } catch (error) {
      console.error(error);
    }
  }

  async function saveSettings(): Promise<void> {
    if (!session) return;
    try {
      await supabase.from("user_settings").upsert({
        user_id: session.user.id,
        language: settings.language,
        clock_style: settings.clockStyle,
        theme_color: settings.themeColor,
        background_url: settings.backgroundUrl,
      });
    } catch (error) {
      console.error(error);
    }
  }

  async function loadEvents(): Promise<void> {
    if (!session) return;
    try {
      const { data, error } = await supabase
        .from("events")
        .select("date_key, event_text")
        .eq("user_id", session.user.id);
      if (data) {
        const newEvents: Record<string, string> = {};
        data.forEach((row) => {
          newEvents[row.date_key] = row.event_text;
        });
        events = newEvents;
      }
    } catch (error) {
      console.error(error);
    }
  }

  async function saveEvent(): Promise<void> {
    if (!selectedDay || !session) return;
    const key = `${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`;

    if (eventText.trim() === "") delete events[key];
    else events[key] = eventText;
    events = { ...events };

    try {
      await supabase
        .from("events")
        .delete()
        .eq("date_key", key)
        .eq("user_id", session.user.id);
      if (eventText.trim() !== "") {
        await supabase.from("events").insert({
          user_id: session.user.id,
          date_key: key,
          event_text: eventText,
        });
      }
    } catch (error) {
      console.error(error);
    }
    closeModal();
  }

  async function deleteEvent(): Promise<void> {
    if (!selectedDay || !session) return;
    const key = `${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`;
    delete events[key];
    events = { ...events };

    try {
      await supabase
        .from("events")
        .delete()
        .eq("date_key", key)
        .eq("user_id", session.user.id);
    } catch (error) {
      console.error(error);
    }
    closeModal();
  }

  // --- LOGIKA KALENDÁŘE A OVLÁDÁNÍ ---
  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Escape") {
      if (selectedDay) closeModal();
      else if (isMenuOpen) {
        isMenuOpen = false;
        setTimeout(() => {
          currentSidebarView = "menu";
        }, 300);
      } else exitApp();
    }
  }

  function handleDayClick(day: number | null): void {
    if (!day) return;
    selectedDay = { day, month: currentMonth, year: currentYear };
    const key = `${currentYear}-${currentMonth}-${day}`;
    eventText = events[key] || "";
  }

  function closeModal(): void {
    selectedDay = null;
  }
  function toggleMenu(): void {
    isMenuOpen = !isMenuOpen;
    if (!isMenuOpen)
      setTimeout(() => {
        currentSidebarView = "menu";
      }, 300);
  }
  async function exitApp(): Promise<void> {
    await invoke("exit_app");
  }

  async function calculateCalendar(): Promise<void> {
    let numberOfDays: number;
    try {
      numberOfDays = await invoke<number>("count_days", {
        monthNo: currentMonth + 1,
        year: currentYear,
      });
    } catch (error) {
      numberOfDays = new Date(currentYear, currentMonth + 1, 0).getDate();
    }

    const firstDayOfMonth = new Date(currentYear, currentMonth, 1).getDay();
    const startOffset = firstDayOfMonth === 0 ? 6 : firstDayOfMonth - 1;

    let tempDays: (number | null)[] = [];
    for (let i = 0; i < startOffset; i++) tempDays.push(null);
    for (let i = 1; i <= numberOfDays; i++) tempDays.push(i);
    daysInMonth = tempDays;
  }

  function goPrevious(): void {
    if (isYearToggleActive) currentYear--;
    else {
      currentMonth--;
      if (currentMonth < 0) {
        currentMonth = 11;
        currentYear--;
      }
    }
    calculateCalendar();
  }

  function goNext(): void {
    if (isYearToggleActive) currentYear++;
    else {
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
    const currentLang = settings.language === "cs" ? "cs-CZ" : "en-US";

    if (settings.clockStyle === "minimal")
      timeText = now.toLocaleTimeString(currentLang, {
        hour: "2-digit",
        minute: "2-digit",
      });
    else
      timeText = now.toLocaleTimeString(currentLang, {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    dateText = now.toLocaleDateString(currentLang, {
      day: "numeric",
      month: "long",
    });

    const hrs = now.getHours();
    const mins = now.getMinutes();
    const secs = now.getSeconds();
    hourAngle = (hrs % 12) * 30 + mins * 0.5;
    minuteAngle = mins * 6 + secs * 0.1;
    secondAngle = secs * 6;
  }

  onMount(() => {
    supabase.auth.getSession().then(({ data }) => {
      session = data.session;
      if (session) {
        loadSettings();
        loadEvents();
      }
    });

    supabase.auth.onAuthStateChange((_event, _session) => {
      session = _session;
      if (session) {
        loadSettings();
        loadEvents();
      }
    });

    updateTime();
    calculateCalendar();
    const interval = setInterval(updateTime, 1000);
    return () => clearInterval(interval);
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<main
  class="app-container palette-{settings.themeColor}"
  style="background-image: url('{settings.backgroundUrl}')"
  data-tauri-drag-region
>
  {#if !session}
    <div class="auth-overlay" data-tauri-drag-region>
      <div class="auth-box" data-tauri-drag-region>
        <h2>{isLoginMode ? "Přihlášení" : "Registrace"}</h2>
        <p class="subtitle">Cloud Calendar Sync</p>

        {#if authError}
          <div class="error-msg">{authError}</div>
        {/if}

        <div class="input-group">
          <label for="email">E-mail</label>
          <input
            id="email"
            type="email"
            bind:value={authEmail}
            placeholder="tvuj@email.cz"
          />
        </div>

        <div class="input-group">
          <label for="password">Heslo</label>
          <input
            id="password"
            type="password"
            bind:value={authPassword}
            placeholder="••••••••"
          />
        </div>

        <button
          class="btn-primary"
          on:click={handleAuth}
          disabled={isAuthLoading}
        >
          {isAuthLoading
            ? "Načítám..."
            : isLoginMode
              ? "Vstoupit"
              : "Vytvořit účet"}
        </button>

        <p class="toggle-auth">
          {isLoginMode ? "Nemáš účet?" : "Už máš účet?"}
          <span
            on:click={() => {
              isLoginMode = !isLoginMode;
              authError = "";
            }}
          >
            {isLoginMode ? "Zaregistruj se" : "Přihlas se"}
          </span>
        </p>

        <button class="btn-exit-small" on:click={exitApp}
          >Zavřít aplikaci</button
        >
      </div>
    </div>
  {:else}
    <button class="settings-btn" on:click={toggleMenu} aria-label="Nastavení">
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <circle cx="12" cy="12" r="3"></circle>
        <path
          d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
        ></path>
      </svg>
    </button>

    <aside class="sidebar" class:open={isMenuOpen}>
      <div class="sidebar-content">
        {#if currentSidebarView === "menu"}
          <h2>{settings.language === "cs" ? "Menu" : "Menu"}</h2>
          <ul class="menu-items">
            <li>
              <button on:click={() => (currentSidebarView = "settings")}
                >{settings.language === "cs" ? "Nastavení" : "Settings"}</button
              >
            </li>
            <li>
              <button on:click={() => (currentSidebarView = "credits")}
                >{settings.language === "cs" ? "Credits" : "Credits"}</button
              >
            </li>
          </ul>
        {:else if currentSidebarView === "settings"}
          <div class="sidebar-header">
            <button
              class="back-btn"
              on:click={() => (currentSidebarView = "menu")}>&#8592;</button
            >
            <h2>{settings.language === "cs" ? "Nastavení" : "Settings"}</h2>
          </div>
          <div class="sidebar-body">
            <div class="settings-group">
              <label for="langSelect"
                >{settings.language === "cs" ? "Jazyk" : "Language"}</label
              >
              <select
                id="langSelect"
                bind:value={settings.language}
                on:change={saveSettings}
              >
                <option value="cs">Čeština</option>
                <option value="en">English</option>
              </select>
            </div>
            <div class="settings-group">
              <label for="clockSelect"
                >{settings.language === "cs"
                  ? "Styl hodin"
                  : "Clock Style"}</label
              >
              <select
                id="clockSelect"
                bind:value={settings.clockStyle}
                on:change={saveSettings}
              >
                <option value="digital">Digital (Full)</option>
                <option value="minimal">Minimal (No Sec)</option>
                <option value="analog">Analog</option>
              </select>
            </div>
            <div class="settings-group">
              <label for="colorSelect"
                >{settings.language === "cs"
                  ? "Barevná paleta"
                  : "Color Palette"}</label
              >
              <select
                id="colorSelect"
                bind:value={settings.themeColor}
                on:change={saveSettings}
              >
                <option value="blue">Windows Blue</option>
                <option value="red">Cyberpunk Red</option>
                <option value="gray">Minimal Gray</option>
                <option value="purple">Vibrant Purple</option>
                <option value="green">Forest Green</option>
              </select>
            </div>
            <div class="settings-group">
              <label for="bgInput"
                >{settings.language === "cs"
                  ? "URL obrázku pozadí"
                  : "Background Image URL"}</label
              >
              <input
                id="bgInput"
                type="text"
                bind:value={settings.backgroundUrl}
                on:change={saveSettings}
                placeholder="https://..."
              />
            </div>
          </div>
        {:else if currentSidebarView === "credits"}
          <div class="sidebar-header">
            <button
              class="back-btn"
              on:click={() => (currentSidebarView = "menu")}>&#8592;</button
            >
            <h2>Credits</h2>
          </div>
          <div class="sidebar-body credits-text">
            <p>Rust coded by me</p>
            <p>Design written by Gemini</p>
            <p>i wont be bothered by typing html and css</p>
            <p style="font-style: italic;">- Bagr Křehňák 2026</p>
          </div>
        {/if}
      </div>

      <div class="bottom-actions">
        <button class="btn-logout" on:click={handleLogout}>
          <svg
            viewBox="0 0 24 24"
            width="18"
            height="18"
            stroke="currentColor"
            stroke-width="2"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path><polyline
              points="10 17 15 12 10 7"
            ></polyline><line x1="15" y1="12" x2="3" y2="12"></line>
          </svg>
          {settings.language === "cs" ? "Odhlásit se" : "Sign Out"}
        </button>

        <button class="btn-exit-app" on:click={exitApp}>
          <svg
            viewBox="0 0 24 24"
            width="18"
            height="18"
            stroke="currentColor"
            stroke-width="2"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline
              points="16 17 21 12 16 7"
            ></polyline><line x1="21" y1="12" x2="9" y2="12"></line>
          </svg>
          {settings.language === "cs" ? "Ukončit aplikaci" : "Exit Application"}
        </button>
      </div>
    </aside>

    <div
      class="sidebar-overlay"
      class:visible={isMenuOpen}
      on:click={toggleMenu}
    ></div>

    <div class="main-widget" data-tauri-drag-region>
      <div class="widget-layout" data-tauri-drag-region>
        <div class="calendar-header" data-tauri-drag-region>
          <button class="nav-btn" on:click={goPrevious}>&#8592;</button>
          <div class="title-toggle">
            <span
              class="month-part"
              class:active={!isYearToggleActive}
              on:click={() => (isYearToggleActive = false)}
            >
              {settings.language === "cs"
                ? monthNamesCS[currentMonth]
                : monthNamesEN[currentMonth]}
            </span>
            <span
              class="year-part"
              class:active={isYearToggleActive}
              on:click={() => (isYearToggleActive = true)}
            >
              {currentYear}
            </span>
          </div>
          <button class="nav-btn" on:click={goNext}>&#8594;</button>
        </div>

        <div class="status-clock" data-tauri-drag-region>
          {#if settings.clockStyle === "analog"}
            <div class="analog-clock-face">
              <div
                class="hand hour-hand"
                style="transform: rotate({hourAngle}deg)"
              ></div>
              <div
                class="hand minute-hand"
                style="transform: rotate({minuteAngle}deg)"
              ></div>
              <div
                class="hand second-hand"
                style="transform: rotate({secondAngle}deg)"
              ></div>
              <div class="center-pin"></div>
            </div>
            <span class="date-small">{dateText}</span>
          {:else}
            <span class="time">{timeText}</span>
            <span class="date-small">{dateText}</span>
          {/if}
        </div>

        <div class="calendar-grid" data-tauri-drag-region>
          {#each settings.language === "cs" ? daysOfWeekCS : daysOfWeekEN as dayName}
            <div class="weekday-label">{dayName}</div>
          {/each}

          {#each daysInMonth as day}
            {@const key = day ? `${currentYear}-${currentMonth}-${day}` : ""}
            <div
              class="day-cell"
              class:today={day === today.getDate() &&
                currentMonth === today.getMonth() &&
                currentYear === today.getFullYear()}
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
            <h3>
              {settings.language === "cs" ? "Poznámka" : "Note"}: {selectedDay.day}.
              {settings.language === "cs"
                ? monthNamesCS[selectedDay.month]
                : monthNamesEN[selectedDay.month]}
              {selectedDay.year}
            </h3>
            <textarea
              bind:value={eventText}
              placeholder={settings.language === "cs"
                ? "Sem zapiš svou událost..."
                : "Type your event here..."}
            ></textarea>
            <div class="modal-actions">
              {#if events[`${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`]}
                <button class="btn-delete" on:click={deleteEvent}
                  >{settings.language === "cs" ? "Vymazat" : "Delete"}</button
                >
              {/if}
              <button class="btn-cancel" on:click={closeModal}
                >{settings.language === "cs" ? "Zrušit" : "Cancel"}</button
              >
              <button class="btn-save" on:click={saveEvent}
                >{settings.language === "cs" ? "Uložit" : "Save"}</button
              >
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
  /* --- AKCENTNÍ PALETY POMOCÍ CSS PROMĚNNÝCH --- */
  .palette-blue {
    --accent-color: #0078d4;
    --accent-glow: rgba(0, 120, 212, 0.5);
  }
  .palette-red {
    --accent-color: #df2a4a;
    --accent-glow: rgba(223, 42, 74, 0.5);
  }
  .palette-gray {
    --accent-color: #e0e0e0;
    --accent-glow: rgba(255, 255, 255, 0.3);
  }
  .palette-purple {
    --accent-color: #a020f0;
    --accent-glow: rgba(160, 32, 240, 0.5);
  }
  .palette-green {
    --accent-color: #107c41;
    --accent-glow: rgba(16, 124, 65, 0.5);
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
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
    background-repeat: no-repeat;
    background-position: center center;
    background-size: cover;
    border-radius: 12px;
    -webkit-app-region: drag;
  }
  button,
  select,
  input,
  .calendar-grid,
  .title-toggle,
  .sidebar,
  .modal-overlay,
  .event-dot,
  .auth-box {
    -webkit-app-region: no-drag;
  }

  /* --- AUTH STYLY --- */
  .auth-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(15px);
    z-index: 100;
  }
  .auth-box {
    background: rgba(30, 30, 30, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 40px;
    border-radius: 20px;
    width: 350px;
    text-align: center;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5);
  }
  .auth-box h2 {
    margin: 0;
    color: white;
    font-size: 2rem;
    font-weight: 500;
  }
  .subtitle {
    color: var(--accent-color);
    margin-top: 5px;
    margin-bottom: 25px;
    font-weight: 600;
    letter-spacing: 1px;
    text-transform: uppercase;
    font-size: 0.85rem;
  }

  .error-msg {
    background: rgba(220, 53, 69, 0.2);
    color: #ff6b6b;
    padding: 10px;
    border-radius: 8px;
    margin-bottom: 20px;
    font-size: 0.9rem;
    border: 1px solid rgba(220, 53, 69, 0.3);
  }

  .input-group {
    text-align: left;
    margin-bottom: 15px;
  }
  .input-group label {
    display: block;
    font-size: 0.85rem;
    opacity: 0.7;
    margin-bottom: 5px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .input-group input {
    width: 100%;
    box-sizing: border-box;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    padding: 12px;
    border-radius: 8px;
    font-size: 1rem;
    transition: 0.2s;
  }
  .input-group input:focus {
    outline: none;
    border-color: var(--accent-color);
    background: rgba(255, 255, 255, 0.1);
  }

  .btn-primary {
    width: 100%;
    background: var(--accent-color);
    color: white;
    border: none;
    padding: 12px;
    border-radius: 8px;
    font-size: 1.1rem;
    font-weight: 600;
    cursor: pointer;
    margin-top: 10px;
    transition: 0.2s;
  }
  .btn-primary:hover {
    filter: brightness(1.2);
  }
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toggle-auth {
    margin-top: 20px;
    font-size: 0.9rem;
    opacity: 0.8;
  }
  .toggle-auth span {
    color: var(--accent-color);
    cursor: pointer;
    font-weight: bold;
    margin-left: 5px;
  }
  .toggle-auth span:hover {
    text-decoration: underline;
  }

  .btn-exit-small {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.4);
    margin-top: 25px;
    cursor: pointer;
    transition: 0.2s;
    font-size: 0.85rem;
  }
  .btn-exit-small:hover {
    color: #ff6b6b;
  }

  /* --- ZBYTEK APLIKACE --- */
  .settings-btn {
    position: absolute;
    top: 20px;
    left: 20px;
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    border-radius: 50%;
    width: 45px;
    height: 45px;
    cursor: pointer;
    z-index: 50;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }
  .settings-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    transform: rotate(45deg);
  }
  .settings-btn svg {
    width: 20px;
    height: 20px;
  }

  .sidebar {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 260px;
    background: rgba(20, 20, 20, 0.85);
    backdrop-filter: blur(30px);
    border-right: 1px solid rgba(255, 255, 255, 0.1);
    transform: translateX(-100%);
    transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 60;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 80px 20px 20px 20px;
    box-sizing: border-box;
  }
  .sidebar.open {
    transform: translateX(0);
  }
  .sidebar-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s ease;
    z-index: 55;
  }
  .sidebar-overlay.visible {
    opacity: 1;
    pointer-events: auto;
  }
  .sidebar h2 {
    margin: 0 0 20px 0;
    font-size: 1.5rem;
    color: var(--accent-color);
    font-weight: 500;
  }
  .sidebar-header {
    display: flex;
    align-items: center;
    gap: 15px;
    margin-bottom: 20px;
  }
  .sidebar-header h2 {
    margin: 0;
  }
  .back-btn {
    background: transparent;
    border: none;
    color: white;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.2s;
  }
  .back-btn:hover {
    transform: translateX(-4px);
  }
  .sidebar-body {
    color: rgba(255, 255, 255, 0.8);
    font-size: 0.95rem;
    line-height: 1.5;
  }
  .credits-text p {
    margin: 8px 0;
    font-style: italic;
  }

  .settings-group {
    margin-bottom: 18px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .settings-group label {
    font-size: 0.85rem;
    opacity: 0.7;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .settings-group select,
  .settings-group input {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    padding: 8px 12px;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
  }
  .settings-group select:focus,
  .settings-group input:focus {
    outline: none;
    border-color: var(--accent-color);
  }
  .settings-group select option {
    background: #141414;
    color: white;
  }

  .menu-items {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .menu-items button {
    width: 100%;
    text-align: left;
    background: transparent;
    border: none;
    color: white;
    font-size: 1.1rem;
    padding: 12px 15px;
    border-radius: 8px;
    cursor: pointer;
    transition: 0.2s;
  }
  .menu-items button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--accent-color);
  }

  .bottom-actions {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .btn-logout {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    font-size: 1rem;
    padding: 12px 15px;
    border-radius: 8px;
    cursor: pointer;
    transition: 0.2s;
  }
  .btn-logout:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .btn-exit-app {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    background: rgba(220, 53, 69, 0.15);
    border: 1px solid rgba(220, 53, 69, 0.3);
    color: #ff6b6b;
    font-size: 1rem;
    padding: 12px 15px;
    border-radius: 8px;
    cursor: pointer;
    transition: 0.2s;
  }
  .btn-exit-app:hover {
    background: rgba(220, 53, 69, 0.9);
    color: white;
  }

  /* TŘÍDA PRO RESPONZIVITU (Desktop verze) */
  .main-widget {
    position: relative;
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(25px);
    -webkit-backdrop-filter: blur(25px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;
    padding: 25px;
    width: 90%;
    max-width: 700px;
    box-shadow: 0 30px 60px rgba(0, 0, 0, 0.5);
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
  .title-toggle span:hover {
    filter: brightness(1.3);
  }
  .month-part {
    font-size: 2.6rem;
    font-weight: 500;
    color: var(--accent-color);
    opacity: 0.35;
  }
  .month-part.active {
    opacity: 1;
  }
  .year-part {
    font-size: 2.3rem;
    font-weight: 300;
    color: white;
    opacity: 0.25;
  }
  .year-part.active {
    opacity: 1;
    font-weight: 500;
  }
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
  .nav-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .status-clock {
    text-align: center;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 12px 25px;
    border-radius: 16px;
    min-width: 160px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .time {
    display: block;
    font-size: 2.2rem;
    font-weight: 500;
    line-height: 1.1;
    font-variant-numeric: tabular-nums;
  }
  .date-small {
    display: block;
    font-size: 0.85rem;
    opacity: 0.6;
    margin-top: 4px;
  }

  .analog-clock-face {
    position: relative;
    width: 65px;
    height: 65px;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 50%;
    margin-bottom: 4px;
    background: rgba(0, 0, 0, 0.1);
  }
  .hand {
    position: absolute;
    bottom: 50%;
    left: 50%;
    transform-origin: bottom center;
    border-radius: 4px;
  }
  .hour-hand {
    width: 3px;
    height: 16px;
    background: white;
    margin-left: -1.5px;
  }
  .minute-hand {
    width: 2px;
    height: 24px;
    background: rgba(255, 255, 255, 0.8);
    margin-left: -1px;
  }
  .second-hand {
    width: 1px;
    height: 26px;
    background: var(--accent-color);
    margin-left: -0.5px;
  }
  .center-pin {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 6px;
    height: 6px;
    background: white;
    border-radius: 50%;
    margin-top: -3px;
    margin-left: -3px;
  }

  .calendar-grid {
    grid-column: span 2;
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 12px;
    text-align: center;
    margin-top: 20px;
  }
  .weekday-label {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--accent-color);
    padding-bottom: 5px;
    text-transform: uppercase;
  }
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
    background: var(--accent-color) !important;
    font-weight: bold;
    color: white;
    box-shadow: 0 0 20px var(--accent-glow);
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
  .today .event-dot {
    background: rgba(255, 255, 255, 0.9);
  }

  .modal-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(8px);
    border-radius: 12px;
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
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
  .modal-content h3 {
    margin: 0 0 15px 0;
    font-weight: 500;
    color: var(--accent-color);
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
    border-color: var(--accent-color);
  }
  .modal-actions {
    display: flex;
    justify-content: flex-end;
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
  .btn-delete {
    margin-right: auto;
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
  .btn-cancel:hover {
    background: rgba(255, 255, 255, 0.2);
  }
  .btn-save {
    background: var(--accent-color);
    color: white;
  }
  .btn-save:hover {
    filter: brightness(1.2);
  }

  /* --- ZDE JSOU NOVÁ RESPONZIVNÍ PRAVIDLA PRO MOBILY --- */
  @media (max-width: 600px) {
    .main-widget {
      width: 95%;
      min-height: 80vh; /* Zázračné pravidlo: Roztáhne to na 80% výšky displeje! */
      padding: 35px 20px;
      display: flex;
      flex-direction: column;
    }

    .widget-layout {
      display: flex;
      flex-direction: column;
      flex: 1; /* Dovolí vnitřku vyplnit celou tu novou výšku */
      justify-content: space-between; /* Rozdělí volný prostor rovnoměrně */
      gap: 20px;
    }

    .calendar-header {
      padding-right: 0;
      justify-content: space-between;
      width: 100%;
    }

    .title-toggle {
      width: auto;
    }

    .month-part {
      font-size: 2.3rem;
    }
    .year-part {
      font-size: 2.1rem;
    }

    .status-clock {
      width: 100%;
      padding: 25px 15px; /* Tlustší bublina pro hodiny */
      box-sizing: border-box;
    }

    .time {
      font-size: 3.2rem;
    } /* Obrovský čas */
    .date-small {
      font-size: 1.1rem;
    }

    .calendar-grid {
      display: grid;
      grid-template-columns: repeat(7, 1fr);
      gap: 8px; /* Větší mezery mezi dny */
      width: 100%;
      margin-top: 10px;
    }

    .day-cell {
      width: 100%;
      height: auto;
      aspect-ratio: 1 / 1.15; /* Lehce obdélníkové = mnohem snazší na trefení */
      border-radius: 14px; /* Místo kruhu moderní zaoblené rohy */
      font-size: 1.3rem; /* Větší čísla */
    }

    .weekday-label {
      font-size: 0.95rem;
      margin-bottom: 12px;
    }

    .auth-box {
      width: 95%;
      padding: 35px 20px;
    }
  }
</style>
