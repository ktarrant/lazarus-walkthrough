<details class="pokemon-card-container">
<summary>Gothitelle (#295)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-gothitelle">
<input type="radio" name="pokemon-tabs-gothitelle-group" id="pokemon-tabs-gothitelle-tab-0">
<label for="pokemon-tabs-gothitelle-tab-0">Gothita</label>
<input type="radio" name="pokemon-tabs-gothitelle-group" id="pokemon-tabs-gothitelle-tab-1">
<label for="pokemon-tabs-gothitelle-tab-1">Gothorita</label>
<input type="radio" name="pokemon-tabs-gothitelle-group" id="pokemon-tabs-gothitelle-tab-2" checked>
<label for="pokemon-tabs-gothitelle-tab-2">Gothitelle</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-gothitelle-panel-0">
Types: Psychic • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Frisk
- Competitive
- Shadow Tag *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM05 - Flash

**Encounter Locations**
- Erinys Path (West) — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gothita" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-low">290</span> |

**Level-Up Moves**
- Pound (Lv 1)
- Confusion (Lv 3)
- Tickle (Lv 7)
- Play Nice (Lv 8)
- Fake Tears (Lv 10)
- Double Slap (Lv 14)
- Psybeam (Lv 16)
- Embargo (Lv 19)
- Feint Attack (Lv 24)
- Psyshock (Lv 25)
- Flatter (Lv 28)
- Relic Song (Lv 30)
- Future Sight (Lv 31)
- Heal Block (Lv 33)
- Psychic (Lv 36)
- Dark Pulse (Lv 39)
- Telekinesis (Lv 41)
- Charm (Lv 46)
- Magic Room (Lv 48)
- Thunderbolt (Lv 52)

**Egg Moves**
- Mirror Coat
- Uproar
- Miracle Eye
- Captivate
- Mean Look
- Dark Pulse
- Heal Pulse

**Tutor Moves**
- Dream Eater
- Endure
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-gothitelle-panel-1">
Types: Psychic • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Frisk
- Competitive
- Shadow Tag *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM05 - Flash

**Evolution Info**
Lv. 27

**Encounter Locations**
- Kaptara Island (West) — Grass (Night) (8%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gothorita" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">390</span> |

**Level-Up Moves**
- Pound (Lv 1)
- Confusion (Lv 3)
- Tickle (Lv 7)
- Play Nice (Lv 8)
- Fake Tears (Lv 10)
- Double Slap (Lv 14)
- Psybeam (Lv 16)
- Embargo (Lv 19)
- Feint Attack (Lv 24)
- Psyshock (Lv 25)
- Flatter (Lv 28)
- Relic Song (Lv 30)
- Future Sight (Lv 31)
- Heal Block (Lv 33)
- Psychic (Lv 36)
- Dark Pulse (Lv 39)
- Telekinesis (Lv 41)
- Charm (Lv 46)
- Magic Room (Lv 48)
- Thunderbolt (Lv 52)

**Egg Moves**
- Mirror Coat
- Uproar
- Miracle Eye
- Captivate
- Mean Look
- Dark Pulse
- Heal Pulse

**Tutor Moves**
- Dream Eater
- Endure
- Metronome
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-gothitelle-panel-2">
Types: Psychic • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Frisk
- Competitive
- Shadow Tag *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM53 - Power-Up Punch
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM05 - Flash

**Evolution Info**
Lv. 41

**Encounter Locations**
- Kaptara Island (West) — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gothitelle" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-high">110</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Perish Song (Lv Evo)
- Pound (Lv 1)
- Confusion (Lv 3)
- Tickle (Lv 7)
- Play Nice (Lv 8)
- Fake Tears (Lv 10)
- Double Slap (Lv 14)
- Psybeam (Lv 16)
- Embargo (Lv 19)
- Feint Attack (Lv 24)
- Psyshock (Lv 25)
- Flatter (Lv 28)
- Relic Song (Lv 30)
- Future Sight (Lv 31)
- Heal Block (Lv 33)
- Psychic (Lv 36)
- Dark Pulse (Lv 39)
- Telekinesis (Lv 41)
- Charm (Lv 46)
- Magic Room (Lv 48)
- Thunderbolt (Lv 52)

**Egg Moves**
- Mirror Coat
- Uproar
- Miracle Eye
- Captivate
- Mean Look
- Dark Pulse
- Heal Pulse

**Tutor Moves**
- Body Slam
- Dream Eater
- Endure
- Metronome
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
</div>
</div>
<style>
#pokemon-tabs-gothitelle-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-gothitelle-panel-0 { display: block; }
#pokemon-tabs-gothitelle-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-gothitelle-panel-1 { display: block; }
#pokemon-tabs-gothitelle-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-gothitelle-panel-2 { display: block; }
</style>
</details>
