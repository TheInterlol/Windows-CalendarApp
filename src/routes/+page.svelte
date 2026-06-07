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
    language: "en" | "cs" | "es" | "zh";
    clockStyle: "digital" | "minimal" | "analog";
    backgroundUrl: string;
    themeColor: string;
    navigationStyle: "swipe" | "arrows";
  }
  type SidebarView = "menu" | "settings" | "credits";

  // --- AUTHENTIKACE A FREEMIUM ---
  let session: any = null;
  let authEmail = "";
  let authPassword = "";
  let isLoginMode = true;
  let isResetMode = false;
  let authError = "";
  let authMessage = "";
  let isAuthLoading = false;

  let isPro = false;
  let showPaywall = false;
  const defaultBg =
    "https://images.unsplash.com/photo-1635776062127-d379bfcba9f8?q=80&w=2000&auto=format&fit=crop";

  let isOffline = false;
  let isMobile = false;
  function handleOffline() {
    isOffline = true;
  }
  function handleOnline() {
    isOffline = false;
  }

  let toastMessage = "";
  let toastTimeout: any;
  function showToast(msg: string) {
    toastMessage = msg;
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => {
      toastMessage = "";
    }, 3000);
  }

  // --- MULTIJAZYČNÝ SLOVNÍK (i18n) ---
  const i18n = {
    en: {
      offline: "You are offline. Cloud sync paused.",
      note: "Note",
      save: "Save",
      cancel: "Cancel",
      del: "Delete",
      saved: "Event saved",
      deleted: "Event deleted",
      settingsSaved: "Settings saved",
      proUnlocked: "👑 PRO unlocked!",
      menu: "Menu",
      settings: "Settings",
      credits: "Credits",
      getPro: "Get PRO Version",
      lang: "Language",
      nav: "Navigation",
      navSwipe: "Swiping (Gestures)",
      navArr: "Buttons (Arrows)",
      clock: "Clock Style",
      color: "Color Palette",
      bgImage: "Background Image",
      customColor: "Custom Color",
      logout: "Sign Out",
      exit: "Exit Application",
      email: "E-mail",
      pwd: "Password",
      forgot: "Forgot password?",
      login: "Login",
      register: "Register",
      enter: "Enter",
      createAcc: "Create account",
      loading: "Loading...",
      noAcc: "Don't have an account?",
      hasAcc: "Already have an account?",
      backLog: "Back to Login",
      sendLink: "Send reset link",
      proTitle: "Unlock Everything Forever",
      proDesc:
        "This feature is only available in the PRO version. Support the dev and get custom colors and backgrounds for a one-time fee.",
      feat1: "Cyberpunk, Green, Purple palettes",
      feat2: "Custom Color Wheel",
      feat3: "Upload custom background images",
      maybeLater: "Maybe later",
      buy: "Buy for 5 €",
      resetSent: "Reset link sent to your e-mail.",
    },
    cs: {
      offline: "Jsi offline. Synchronizace pozastavena.",
      note: "Poznámka",
      save: "Uložit",
      cancel: "Zrušit",
      del: "Vymazat",
      saved: "Událost uložena",
      deleted: "Událost smazána",
      settingsSaved: "Nastavení uloženo",
      proUnlocked: "👑 PRO odemknuto!",
      menu: "Menu",
      settings: "Nastavení",
      credits: "Credits",
      getPro: "Získat PRO verzi",
      lang: "Jazyk",
      nav: "Ovládání",
      navSwipe: "Swipování (Gesta)",
      navArr: "Tlačítka (Šipky)",
      clock: "Styl hodin",
      color: "Barevná paleta",
      bgImage: "Obrázek pozadí",
      customColor: "Vlastní barva",
      logout: "Odhlásit se",
      exit: "Ukončit aplikaci",
      email: "E-mail",
      pwd: "Heslo",
      forgot: "Zapomněl(a) jsi heslo?",
      login: "Přihlášení",
      register: "Registrace",
      enter: "Vstoupit",
      createAcc: "Vytvořit účet",
      loading: "Načítám...",
      noAcc: "Nemáš účet?",
      hasAcc: "Už máš účet?",
      backLog: "Zpět na Přihlášení",
      sendLink: "Odeslat odkaz",
      proTitle: "Odemkni si vše navždy",
      proDesc:
        "Tato funkce je dostupná pouze v PRO verzi. Podpoř vývoj a získej všechny barvy a vlastní pozadí za jednorázovou platbu.",
      feat1: "Cyberpunk, Green, Purple palety",
      feat2: "Vlastní výběr barev z palety",
      feat3: "Nahrávání vlastních obrázků",
      maybeLater: "Možná později",
      buy: "Koupit za 125 Kč",
      resetSent: "Odkaz byl odeslán na tvůj e-mail.",
    },
    es: {
      offline: "Estás desconectado. Sincronización pausada.",
      note: "Nota",
      save: "Guardar",
      cancel: "Cancelar",
      del: "Borrar",
      saved: "Evento guardado",
      deleted: "Evento borrado",
      settingsSaved: "Ajustes guardados",
      proUnlocked: "👑 ¡PRO desbloqueado!",
      menu: "Menú",
      settings: "Ajustes",
      credits: "Créditos",
      getPro: "Obtener versión PRO",
      lang: "Idioma",
      nav: "Navegación",
      navSwipe: "Deslizar (Gestos)",
      navArr: "Botones (Flechas)",
      clock: "Estilo de reloj",
      color: "Paleta de colores",
      bgImage: "Imagen de fondo",
      customColor: "Color personalizado",
      logout: "Cerrar sesión",
      exit: "Salir",
      email: "Correo",
      pwd: "Contraseña",
      forgot: "¿Olvidaste tu contraseña?",
      login: "Iniciar sesión",
      register: "Registrarse",
      enter: "Entrar",
      createAcc: "Crear cuenta",
      loading: "Cargando...",
      noAcc: "¿No tienes cuenta?",
      hasAcc: "¿Ya tienes cuenta?",
      backLog: "Volver al inicio",
      sendLink: "Enviar enlace",
      proTitle: "Desbloquea Todo Para Siempre",
      proDesc:
        "Esta función solo está disponible en la versión PRO. Apoya al desarrollador y obtén colores y fondos personalizados por un pago único.",
      feat1: "Paletas Cyberpunk, Green, Purple",
      feat2: "Rueda de colores personalizada",
      feat3: "Sube imágenes de fondo personalizadas",
      maybeLater: "Quizás más tarde",
      buy: "Comprar por 5 €",
      resetSent: "Enlace enviado a tu correo.",
    },
    zh: {
      offline: "您处于离线状态。云同步已暂停。",
      note: "笔记",
      save: "保存",
      cancel: "取消",
      del: "删除",
      saved: "事件已保存",
      deleted: "事件已删除",
      settingsSaved: "设置已保存",
      proUnlocked: "👑 PRO 已解锁！",
      menu: "菜单",
      settings: "设置",
      credits: "关于",
      getPro: "获取 PRO 版本",
      lang: "语言",
      nav: "导航",
      navSwipe: "滑动 (手势)",
      navArr: "按钮 (箭头)",
      clock: "时钟样式",
      color: "颜色主题",
      bgImage: "背景图片",
      customColor: "自定义颜色",
      logout: "登出",
      exit: "退出应用",
      email: "电子邮件",
      pwd: "密码",
      forgot: "忘记密码？",
      login: "登录",
      register: "注册",
      enter: "进入",
      createAcc: "创建账户",
      loading: "加载中...",
      noAcc: "没有账户？",
      hasAcc: "已有账户？",
      backLog: "返回登录",
      sendLink: "发送重置链接",
      proTitle: "永久解锁所有功能",
      proDesc:
        "此功能仅在 PRO 版本中可用。支持开发者并支付一次性费用即可获取自定义颜色和背景。",
      feat1: "赛博朋克、绿色、紫色主题",
      feat2: "自定义颜色选择器",
      feat3: "上传自定义背景图片",
      maybeLater: "以后再说",
      buy: "购买 (5 €)",
      resetSent: "重置链接已发送至您的邮箱。",
    },
  };

  const daysOfWeek = {
    en: ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
    cs: ["Po", "Út", "St", "Čt", "Pá", "So", "Ne"],
    es: ["Lu", "Ma", "Mi", "Ju", "Vi", "Sá", "Do"],
    zh: ["一", "二", "三", "四", "五", "六", "日"],
  };

  const monthNames = {
    en: [
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
    ],
    cs: [
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
    ],
    es: [
      "Enero",
      "Febrero",
      "Marzo",
      "Abril",
      "Mayo",
      "Junio",
      "Julio",
      "Agosto",
      "Septiembre",
      "Octubre",
      "Noviembre",
      "Diciembre",
    ],
    zh: [
      "一月",
      "二月",
      "三月",
      "四月",
      "五月",
      "六月",
      "七月",
      "八月",
      "九月",
      "十月",
      "十一月",
      "十二月",
    ],
  };

  // --- REAKTIVNÍ STAV APLIKACE ---
  let timeText = "";
  let dateText = "";
  let hourAngle = 0;
  let minuteAngle = 0;
  let secondAngle = 0;
  let today = new Date();
  let currentMonth = today.getMonth();
  let currentYear = today.getFullYear();
  let isYearToggleActive = false;

  let settings: AppSettings = {
    language: "en", // VÝCHOZÍ ANGLIČTINA
    clockStyle: "digital",
    themeColor: "blue",
    navigationStyle: "swipe",
    backgroundUrl: defaultBg,
  };

  $: t = i18n[settings.language]; // Reaktivní slovník

  // --- BARVY A COLOR WHEEL ---
  const presetColors: Record<string, string> = {
    blue: "#0078d4",
    gray: "#7e7e7e",
    red: "#df2a4a",
    purple: "#a020f0",
    green: "#107c41",
  };
  let selectedThemeOption = "blue";
  let customColorHex = "#ffaa00";

  $: activeAccentColor =
    presetColors[settings.themeColor] || settings.themeColor;

  let daysInMonth: (number | null)[] = [];
  let events: Record<string, string> = {};
  let selectedDay: SelectedDay | null = null;
  let eventText = "";
  let isMenuOpen = false;
  let currentSidebarView: SidebarView = "menu";

  let fileInput: HTMLInputElement; // Pro nahrávání fotek

  // --- DETEKCE SWIPOVÁNÍ ---
  let touchStartX = 0;
  let touchEndX = 0;
  function handleTouchStart(e: TouchEvent) {
    if (!isMobile || settings.navigationStyle !== "swipe") return;
    touchStartX = e.changedTouches[0].screenX;
  }
  function handleTouchEnd(e: TouchEvent) {
    if (!isMobile || settings.navigationStyle !== "swipe") return;
    touchEndX = e.changedTouches[0].screenX;
    handleSwipe();
  }
  function handleSwipe() {
    const threshold = 50;
    if (touchEndX < touchStartX - threshold) goNext();
    if (touchEndX > touchStartX + threshold) goPrevious();
  }

  // --- AUTH LOGIKA ---
  async function handleAuth() {
    isAuthLoading = true;
    authError = "";
    authMessage = "";
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

  async function handleResetPassword() {
    isAuthLoading = true;
    authError = "";
    authMessage = "";
    const { error } = await supabase.auth.resetPasswordForEmail(authEmail);
    if (error) authError = error.message;
    else authMessage = t.resetSent;
    isAuthLoading = false;
  }

  async function handleLogout() {
    await supabase.auth.signOut();
    events = {};
    isMenuOpen = false;
    isPro = false;
  }

  // --- CLOUD NAČÍTÁNÍ A UKLÁDÁNÍ ---
  async function loadSettings() {
    if (!session || isOffline) return;
    try {
      const { data } = await supabase
        .from("user_settings")
        .select("*")
        .eq("user_id", session.user.id)
        .maybeSingle();
      if (data) {
        settings.language = data.language || "en";
        settings.clockStyle = data.clock_style || "digital";
        settings.themeColor = data.theme_color || "blue";
        settings.backgroundUrl = data.background_url || defaultBg;
        settings.navigationStyle = data.navigation_style || "swipe";
        isPro = data.is_pro === true;

        if (presetColors[settings.themeColor]) {
          selectedThemeOption = settings.themeColor;
        } else {
          selectedThemeOption = "custom";
          customColorHex = settings.themeColor;
        }
      }
    } catch (e) {}
  }

  async function saveSettings(silent = false) {
    if (!session || isOffline) return;
    try {
      await supabase.from("user_settings").upsert({
        user_id: session.user.id,
        language: settings.language,
        clock_style: settings.clockStyle,
        theme_color: settings.themeColor,
        background_url: settings.backgroundUrl,
        navigation_style: settings.navigationStyle,
        is_pro: isPro,
      });
      if (!silent) showToast(t.settingsSaved);
    } catch (e) {}
  }

  // --- FREEMIUM LOGIKA BAREV A OBRÁZKŮ ---
  function handleThemeOptionChange() {
    if (
      ["red", "purple", "green", "custom"].includes(selectedThemeOption) &&
      !isPro
    ) {
      selectedThemeOption = "blue";
      settings.themeColor = "blue";
      showPaywall = true;
      return;
    }
    settings.themeColor =
      selectedThemeOption === "custom" ? customColorHex : selectedThemeOption;
    saveSettings();
  }

  function handleCustomColorInput() {
    if (selectedThemeOption === "custom") {
      settings.themeColor = customColorHex;
      saveSettings(true);
    }
  }

  function handleBgChange() {
    if (
      !isPro &&
      settings.backgroundUrl !== defaultBg &&
      settings.backgroundUrl.trim() !== ""
    ) {
      settings.backgroundUrl = defaultBg;
      showPaywall = true;
      return;
    }
    saveSettings();
  }

  function triggerFileUpload() {
    if (!isPro) {
      showPaywall = true;
      return;
    }
    fileInput.click();
  }

  function handleFileUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;
    const file = target.files[0];
    const reader = new FileReader();
    reader.onload = (res) => {
      settings.backgroundUrl = res.target?.result as string;
      saveSettings();
    };
    reader.readAsDataURL(file); // Převod na Base64
  }

  function simulatePurchase() {
    isPro = true;
    showPaywall = false;
    showToast(t.proUnlocked);
    saveSettings(true);
  }

  // --- LOGIKA KALENDÁŘE ---
  async function loadEvents() {
    if (!session || isOffline) return;
    try {
      const { data } = await supabase
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
    } catch (e) {}
  }

  async function saveEvent() {
    if (!selectedDay || !session || isOffline) return;
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
      if (eventText.trim() !== "")
        await supabase
          .from("events")
          .insert({
            user_id: session.user.id,
            date_key: key,
            event_text: eventText,
          });
      showToast(t.saved);
    } catch (e) {}
    closeModal();
  }

  async function deleteEvent() {
    if (!selectedDay || !session || isOffline) return;
    const key = `${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`;
    delete events[key];
    events = { ...events };
    try {
      await supabase
        .from("events")
        .delete()
        .eq("date_key", key)
        .eq("user_id", session.user.id);
      showToast(t.deleted);
    } catch (e) {}
    closeModal();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      if (showPaywall) showPaywall = false;
      else if (selectedDay) closeModal();
      else if (isMenuOpen) {
        isMenuOpen = false;
        setTimeout(() => {
          currentSidebarView = "menu";
        }, 300);
      } else exitApp();
    }
  }

  function handleDayClick(day: number | null) {
    if (!day) return;
    selectedDay = { day, month: currentMonth, year: currentYear };
    eventText = events[`${currentYear}-${currentMonth}-${day}`] || "";
  }

  function closeModal() {
    selectedDay = null;
  }
  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
    if (!isMenuOpen)
      setTimeout(() => {
        currentSidebarView = "menu";
      }, 300);
  }
  async function exitApp() {
    await invoke("exit_app");
  }

  async function calculateCalendar() {
    let numberOfDays: number;
    try {
      numberOfDays = await invoke<number>("count_days", {
        monthNo: currentMonth + 1,
        year: currentYear,
      });
    } catch (e) {
      numberOfDays = new Date(currentYear, currentMonth + 1, 0).getDate();
    }
    const firstDayOfMonth = new Date(currentYear, currentMonth, 1).getDay();
    const startOffset = firstDayOfMonth === 0 ? 6 : firstDayOfMonth - 1;
    let tempDays: (number | null)[] = [];
    for (let i = 0; i < startOffset; i++) tempDays.push(null);
    for (let i = 1; i <= numberOfDays; i++) tempDays.push(i);
    daysInMonth = tempDays;
  }

  function goPrevious() {
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
  function goNext() {
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

  function updateTime() {
    const now = new Date();
    // Volba lokality pro formát času
    let loc = "en-US";
    if (settings.language === "cs") loc = "cs-CZ";
    if (settings.language === "es") loc = "es-ES";
    if (settings.language === "zh") loc = "zh-CN";
    if (settings.clockStyle === "minimal")
      timeText = now.toLocaleTimeString(loc, {
        hour: "2-digit",
        minute: "2-digit",
      });
    else
      timeText = now.toLocaleTimeString(loc, {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    dateText = now.toLocaleDateString(loc, { day: "numeric", month: "long" });
    const hrs = now.getHours();
    const mins = now.getMinutes();
    const secs = now.getSeconds();
    hourAngle = (hrs % 12) * 30 + mins * 0.5;
    minuteAngle = mins * 6 + secs * 0.1;
    secondAngle = secs * 6;
  }

  onMount(() => {
    isOffline = !navigator.onLine;
    isMobile =
      navigator.maxTouchPoints > 0 ||
      window.matchMedia("(pointer: coarse)").matches;
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

<svelte:window
  on:keydown={handleKeydown}
  on:offline={handleOffline}
  on:online={handleOnline}
/>

<main
  class="app-container"
  style="background-image: url('{settings.backgroundUrl}'); --accent-color: {activeAccentColor}; --accent-glow: {activeAccentColor}80;"
  data-tauri-drag-region
>
  {#if isOffline}
    <div class="offline-banner">
      <svg
        viewBox="0 0 24 24"
        width="18"
        height="18"
        stroke="currentColor"
        stroke-width="2"
        fill="none"
        ><path d="M1 1l22 22"></path><path
          d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"
        ></path><path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"></path><path
          d="M10.71 5.05A16 16 0 0 1 22.58 9"
        ></path><path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"></path><path
          d="M8.53 16.11a6 6 0 0 1 6.95 0"
        ></path><line x1="12" y1="20" x2="12.01" y2="20"></line></svg
      >
      {t.offline}
    </div>
  {/if}

  {#if toastMessage}
    <div class="toast-notification">
      <svg
        viewBox="0 0 24 24"
        width="18"
        height="18"
        stroke="currentColor"
        stroke-width="2"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline
          points="22 4 12 14.01 9 11.01"
        ></polyline></svg
      >
      {toastMessage}
    </div>
  {/if}

  {#if showPaywall}
    <div class="modal-overlay" on:click={() => (showPaywall = false)}>
      <div class="modal-content paywall-box" on:click|stopPropagation>
        <div class="pro-badge-large">👑 PRO</div>
        <h3>{t.proTitle}</h3>
        <p class="paywall-desc">{t.proDesc}</p>
        <ul class="pro-features">
          <li>✔️ {t.feat1}</li>
          <li>✔️ {t.feat2}</li>
          <li>✔️ {t.feat3}</li>
        </ul>
        <div class="modal-actions paywall-actions">
          <button class="btn-cancel" on:click={() => (showPaywall = false)}
            >{t.maybeLater}</button
          >
          <button class="btn-primary" on:click={simulatePurchase}
            >{t.buy}</button
          >
        </div>
      </div>
    </div>
  {/if}

  {#if !session}
    <div class="auth-overlay" data-tauri-drag-region>
      <div class="auth-box" data-tauri-drag-region>
        {#if isResetMode}
          <h2>{t.forgot}</h2>
          <p class="subtitle">Chroniq</p>
          {#if authError}
            <div class="error-msg">{authError}</div>
          {/if}
          {#if authMessage}
            <div class="success-msg">{authMessage}</div>
          {/if}
          <div class="input-group">
            <label for="email">{t.email}</label>
            <input
              id="email"
              type="email"
              bind:value={authEmail}
              placeholder="tvuj@email.com"
            />
          </div>
          <button
            class="btn-primary"
            on:click={handleResetPassword}
            disabled={isAuthLoading || !authEmail || isOffline}
            >{isAuthLoading ? t.loading : t.sendLink}</button
          >
          <p class="toggle-auth">
            <span
              on:click={() => {
                isResetMode = false;
                authError = "";
                authMessage = "";
              }}>{t.backLog}</span
            >
          </p>
        {:else}
          <h2>{isLoginMode ? t.login : t.register}</h2>
          <p class="subtitle">Chroniq</p>
          {#if authError}
            <div class="error-msg">{authError}</div>
          {/if}
          <div class="input-group">
            <label for="email">{t.email}</label>
            <input
              id="email"
              type="email"
              bind:value={authEmail}
              placeholder="tvuj@email.com"
            />
          </div>
          <div class="input-group">
            <label for="password">{t.pwd}</label>
            <input
              id="password"
              type="password"
              bind:value={authPassword}
              placeholder="••••••••"
            />
            {#if isLoginMode}
              <div
                class="forgot-pwd"
                on:click={() => {
                  isResetMode = true;
                  authError = "";
                  authMessage = "";
                }}
              >
                {t.forgot}
              </div>
            {/if}
          </div>
          <button
            class="btn-primary"
            on:click={handleAuth}
            disabled={isAuthLoading || isOffline}
            >{isAuthLoading
              ? t.loading
              : isLoginMode
                ? t.enter
                : t.createAcc}</button
          >
          <p class="toggle-auth">
            {isLoginMode ? t.noAcc : t.hasAcc}
            <span
              on:click={() => {
                isLoginMode = !isLoginMode;
                authError = "";
              }}>{isLoginMode ? t.register : t.login}</span
            >
          </p>
        {/if}
        <button class="btn-exit-small" on:click={exitApp}>{t.exit}</button>
      </div>
    </div>
  {:else}
    <button class="settings-btn" on:click={toggleMenu} aria-label="Settings">
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><circle cx="12" cy="12" r="3"></circle><path
          d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
        ></path></svg
      >
    </button>

    <aside class="sidebar" class:open={isMenuOpen}>
      <div class="sidebar-content">
        {#if currentSidebarView === "menu"}
          <h2>{t.menu}</h2>
          <ul class="menu-items">
            <li>
              <button on:click={() => (currentSidebarView = "settings")}
                >{t.settings}</button
              >
            </li>
            <li>
              <button on:click={() => (currentSidebarView = "credits")}
                >{t.credits}</button
              >
            </li>
            {#if !isPro}
              <li style="margin-top: 15px;">
                <button class="btn-pro" on:click={() => (showPaywall = true)}
                  >👑 {t.getPro}</button
                >
              </li>
            {/if}
          </ul>
        {:else if currentSidebarView === "settings"}
          <div class="sidebar-header">
            <button
              class="back-btn"
              on:click={() => (currentSidebarView = "menu")}>&#8592;</button
            >
            <h2>{t.settings}</h2>
          </div>
          <div class="sidebar-body">
            <div class="settings-group">
              <label for="langSelect">{t.lang}</label>
              <select
                id="langSelect"
                bind:value={settings.language}
                on:change={() => saveSettings()}
                disabled={isOffline}
              >
                <option value="en">English</option>
                <option value="cs">Čeština</option>
                <option value="es">Español</option>
                <option value="zh">中文</option>
              </select>
            </div>

            {#if isMobile}
              <div class="settings-group">
                <label for="navSelect">{t.nav}</label>
                <select
                  id="navSelect"
                  bind:value={settings.navigationStyle}
                  on:change={() => saveSettings()}
                  disabled={isOffline}
                >
                  <option value="swipe">{t.navSwipe}</option>
                  <option value="arrows">{t.navArr}</option>
                </select>
              </div>
            {/if}

            <div class="settings-group">
              <label for="clockSelect">{t.clock}</label>
              <select
                id="clockSelect"
                bind:value={settings.clockStyle}
                on:change={() => saveSettings()}
                disabled={isOffline}
              >
                <option value="digital">Digital (Full)</option>
                <option value="minimal">Minimal (No Sec)</option>
                <option value="analog">Analog</option>
              </select>
            </div>

            <div class="settings-group">
              <label for="colorSelect">{t.color}</label>
              <select
                id="colorSelect"
                bind:value={selectedThemeOption}
                on:change={handleThemeOptionChange}
                disabled={isOffline}
              >
                <option value="blue">Blue (Default)</option>
                <option value="gray">Minimal Gray</option>
                <option value="red">Cyberpunk Red 👑</option>
                <option value="purple">Vibrant Purple 👑</option>
                <option value="green">Forest Green 👑</option>
                <option value="custom">{t.customColor} 👑</option>
              </select>
              {#if selectedThemeOption === "custom"}
                <input
                  type="color"
                  class="color-picker"
                  bind:value={customColorHex}
                  on:change={handleCustomColorInput}
                  disabled={isOffline}
                />
              {/if}
            </div>

            <div class="settings-group">
              <label for="bgInput">{t.bgImage} 👑</label>
              <div class="bg-input-row">
                <input
                  id="bgInput"
                  type="text"
                  bind:value={settings.backgroundUrl}
                  on:change={handleBgChange}
                  placeholder="https://..."
                  disabled={isOffline}
                />
                <button
                  class="btn-file"
                  on:click={triggerFileUpload}
                  disabled={isOffline}
                  title="Upload Image">🖼️</button
                >
              </div>
              <input
                type="file"
                accept="image/*"
                bind:this={fileInput}
                on:change={handleFileUpload}
                style="display: none;"
              />
            </div>
          </div>
        {:else if currentSidebarView === "credits"}
          <div class="sidebar-header">
            <button
              class="back-btn"
              on:click={() => (currentSidebarView = "menu")}>&#8592;</button
            >
            <h2>{t.credits}</h2>
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
        <button class="btn-logout" on:click={handleLogout} disabled={isOffline}
          ><svg
            viewBox="0 0 24 24"
            width="18"
            height="18"
            stroke="currentColor"
            stroke-width="2"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
            ><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"
            ></path><polyline points="10 17 15 12 10 7"></polyline><line
              x1="15"
              y1="12"
              x2="3"
              y2="12"
            ></line></svg
          >{t.logout}</button
        >
        <button class="btn-exit-app" on:click={exitApp}
          ><svg
            viewBox="0 0 24 24"
            width="18"
            height="18"
            stroke="currentColor"
            stroke-width="2"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
            ><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline
              points="16 17 21 12 16 7"
            ></polyline><line x1="21" y1="12" x2="9" y2="12"></line></svg
          >{t.exit}</button
        >
      </div>
    </aside>

    <div
      class="sidebar-overlay"
      class:visible={isMenuOpen}
      on:click={toggleMenu}
    ></div>

    <div class="main-widget" data-tauri-drag-region>
      <div
        class="widget-layout"
        data-tauri-drag-region
        on:touchstart={handleTouchStart}
        on:touchend={handleTouchEnd}
      >
        <div class="calendar-header" data-tauri-drag-region>
          {#if !isMobile || settings.navigationStyle === "arrows"}
            <button class="nav-btn" on:click={goPrevious}>&#8592;</button>
          {:else}
            <div class="nav-placeholder"></div>
          {/if}
          <div class="title-toggle">
            <span
              class="month-part"
              class:active={!isYearToggleActive}
              on:click={() => (isYearToggleActive = false)}
              >{monthNames[settings.language][currentMonth]}</span
            >
            <span
              class="year-part"
              class:active={isYearToggleActive}
              on:click={() => (isYearToggleActive = true)}>{currentYear}</span
            >
          </div>
          {#if !isMobile || settings.navigationStyle === "arrows"}
            <button class="nav-btn" on:click={goNext}>&#8594;</button>
          {:else}
            <div class="nav-placeholder"></div>
          {/if}
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
            <span class="time">{timeText}</span><span class="date-small"
              >{dateText}</span
            >
          {/if}
        </div>

        <div class="calendar-grid" data-tauri-drag-region>
          {#each daysOfWeek[settings.language] as dayName}
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
              {t.note}: {selectedDay.day}. {monthNames[settings.language][
                selectedDay.month
              ]}
              {selectedDay.year}
            </h3>
            <textarea bind:value={eventText} placeholder="..."></textarea>
            <div class="modal-actions">
              {#if events[`${selectedDay.year}-${selectedDay.month}-${selectedDay.day}`]}
                <button
                  class="btn-delete"
                  on:click={deleteEvent}
                  disabled={isOffline}>{t.del}</button
                >
              {/if}
              <button class="btn-cancel" on:click={closeModal}
                >{t.cancel}</button
              >
              <button class="btn-save" on:click={saveEvent} disabled={isOffline}
                >{isOffline ? "Offline" : t.save}</button
              >
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
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
    transition: background-image 0.5s ease;
  }
  button,
  select,
  input,
  .calendar-grid,
  .title-toggle,
  .sidebar,
  .modal-overlay,
  .event-dot,
  .auth-box,
  .bg-input-row,
  .color-picker {
    -webkit-app-region: no-drag;
  }

  .offline-banner {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    background: rgba(255, 152, 0, 0.9);
    backdrop-filter: blur(10px);
    color: white;
    text-align: center;
    padding: 10px;
    font-size: 0.95rem;
    font-weight: 600;
    z-index: 200;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
    box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
  }
  .toast-notification {
    position: absolute;
    bottom: 30px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(30, 30, 30, 0.95);
    border: 1px solid var(--accent-color);
    color: white;
    padding: 12px 24px;
    border-radius: 50px;
    font-size: 1rem;
    font-weight: 500;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
    z-index: 1000;
    display: flex;
    align-items: center;
    gap: 10px;
    animation: slideUp 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }
  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translate(-50%, 20px) scale(0.9);
    }
    to {
      opacity: 1;
      transform: translate(-50%, 0) scale(1);
    }
  }

  .paywall-box {
    text-align: center;
    width: 350px;
  }
  .pro-badge-large {
    background: linear-gradient(135deg, #f6d365 0%, #fda085 100%);
    color: black;
    display: inline-block;
    padding: 6px 15px;
    border-radius: 20px;
    font-weight: 800;
    font-size: 0.9rem;
    margin-bottom: 10px;
    text-transform: uppercase;
    letter-spacing: 1px;
  }
  .paywall-desc {
    font-size: 0.95rem;
    line-height: 1.5;
    color: rgba(255, 255, 255, 0.8);
    margin-bottom: 20px;
  }
  .pro-features {
    list-style: none;
    padding: 0;
    margin: 0 0 25px 0;
    text-align: left;
    background: rgba(0, 0, 0, 0.2);
    padding: 15px;
    border-radius: 12px;
  }
  .pro-features li {
    margin-bottom: 10px;
    font-size: 0.9rem;
    color: #fff;
    display: flex;
    gap: 10px;
  }
  .pro-features li:last-child {
    margin-bottom: 0;
  }
  .paywall-actions {
    flex-direction: column;
    gap: 10px;
  }
  .paywall-actions .btn-primary {
    width: 100%;
    margin: 0;
    background: linear-gradient(135deg, #f6d365 0%, #fda085 100%);
    color: #000;
  }
  .paywall-actions .btn-primary:hover {
    transform: scale(1.02);
  }
  .paywall-actions .btn-cancel {
    margin: 0;
    width: 100%;
    background: transparent;
  }
  .btn-pro {
    background: linear-gradient(
      135deg,
      rgba(246, 211, 101, 0.1) 0%,
      rgba(253, 160, 133, 0.1) 100%
    ) !important;
    border: 1px solid rgba(246, 211, 101, 0.3) !important;
    color: #f6d365 !important;
    font-weight: 600;
  }
  .btn-pro:hover {
    background: linear-gradient(
      135deg,
      rgba(246, 211, 101, 0.2) 0%,
      rgba(253, 160, 133, 0.2) 100%
    ) !important;
  }

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
  .success-msg {
    background: rgba(40, 167, 69, 0.2);
    color: #28a745;
    padding: 10px;
    border-radius: 8px;
    margin-bottom: 20px;
    font-size: 0.9rem;
    border: 1px solid rgba(40, 167, 69, 0.3);
  }
  .forgot-pwd {
    text-align: right;
    font-size: 0.8rem;
    color: var(--accent-color);
    margin-top: 5px;
    cursor: pointer;
    opacity: 0.8;
    transition: 0.2s;
  }
  .forgot-pwd:hover {
    opacity: 1;
    text-decoration: underline;
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
  .input-group input,
  .settings-group select,
  .settings-group input {
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
  .input-group input:focus,
  .settings-group select:focus,
  .settings-group input:focus {
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
  .btn-primary:hover:not(:disabled) {
    filter: brightness(1.2);
  }
  .btn-primary:disabled,
  button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    filter: grayscale(100%);
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
  .settings-group select option {
    background: #141414;
    color: white;
  }
  .settings-group select,
  .settings-group input {
    padding: 8px 12px;
  }

  /* NOVÉ STYLY PRO FILE UPLOAD A COLOR PICKER */
  .color-picker {
    margin-top: 5px;
    height: 40px;
    padding: 2px;
    cursor: pointer;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: transparent;
  }
  .bg-input-row {
    display: flex;
    gap: 8px;
  }
  .btn-file {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: white;
    border-radius: 8px;
    padding: 0 12px;
    cursor: pointer;
    transition: 0.2s;
  }
  .btn-file:hover:not(:disabled) {
    background: var(--accent-color);
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
  .btn-logout:hover:not(:disabled) {
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
  .nav-placeholder {
    width: 42px;
    flex-shrink: 0;
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
    line-height: 1;
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
    bottom: 15%;
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
  .btn-delete:hover:not(:disabled) {
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
  .btn-save:hover:not(:disabled) {
    filter: brightness(1.2);
  }

  @media (max-width: 600px) {
    .main-widget {
      width: 95%;
      min-height: 80vh;
      padding: 35px 20px;
      display: flex;
      flex-direction: column;
    }
    .widget-layout {
      display: flex;
      flex-direction: column;
      flex: 1;
      justify-content: space-between;
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
      padding: 25px 15px;
      box-sizing: border-box;
    }
    .time {
      font-size: 3.2rem;
    }
    .date-small {
      font-size: 1.1rem;
    }
    .calendar-grid {
      display: grid;
      grid-template-columns: repeat(7, 1fr);
      gap: 8px;
      width: 100%;
      margin-top: 10px;
    }
    .day-cell {
      width: 100%;
      height: auto;
      aspect-ratio: 1 / 1.15;
      border-radius: 14px;
      font-size: 1.3rem;
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
