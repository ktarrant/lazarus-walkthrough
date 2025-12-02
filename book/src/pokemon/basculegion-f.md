<details class="pokemon-card-container">
<summary>Basculegion♀ (#288)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-basculegion-f">
<input type="radio" name="pokemon-tabs-basculegion-f-group" id="pokemon-tabs-basculegion-f-tab-0">
<label for="pokemon-tabs-basculegion-f-tab-0">Basculin White</label>
<input type="radio" name="pokemon-tabs-basculegion-f-group" id="pokemon-tabs-basculegion-f-tab-1">
<label for="pokemon-tabs-basculegion-f-tab-1">Basculegion♂</label>
<input type="radio" name="pokemon-tabs-basculegion-f-group" id="pokemon-tabs-basculegion-f-tab-2" checked>
<label for="pokemon-tabs-basculegion-f-tab-2">Basculegion♀</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-basculegion-f-panel-0">
Types: Water • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Rattled
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM42 - Facade
- TM44 - Rest
- HM03 - Surf
- HM07 - Waterfall

**Held Item**
Deep Sea Scale
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculin-white" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">92</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-high">98</span> |
| Total | <span class="stat-value stat-mid">460</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Tail Whip (Lv 1)
- Water Gun (Lv 1)
- Uproar (Lv 3)
- Headbutt (Lv 5)
- Bite (Lv 7)
- Aqua Jet (Lv 9)
- Chip Away (Lv 11)
- Take Down (Lv 14)
- Crunch (Lv 17)
- Aqua Tail (Lv 20)
- Soak (Lv 23)
- Double-Edge (Lv 26)
- Scary Face (Lv 28)
- Wave Crash (Lv 32)
- Flail (Lv 34)
- Final Gambit (Lv 38)
- Zen Headbutt (Lv 40)
- Thrash (Lv 42)
- Head Smash (Lv 46)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-basculegion-f-panel-1">
Types: Water / Ghost • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swift Swim
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM29 - Psychic
- TM30 - Shadow Ball
- TM42 - Facade
- TM44 - Rest
- HM03 - Surf
- HM07 - Waterfall

**Evolution Info**
Lv. after 294 Recoil Dmg, Male
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculegion-m" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">120</span> |
| Attack | <span class="stat-value stat-high">112</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-mid">78</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Bitter Malice (Lv Evo)
- Shadow Force (Lv Evo)
- Tackle (Lv 1)
- Tail Whip (Lv 1)
- Water Gun (Lv 1)
- Uproar (Lv 3)
- Headbutt (Lv 5)
- Bite (Lv 7)
- Aqua Jet (Lv 9)
- Chip Away (Lv 11)
- Take Down (Lv 14)
- Crunch (Lv 17)
- Aqua Tail (Lv 20)
- Soak (Lv 23)
- Double-Edge (Lv 26)
- Scary Face (Lv 28)
- Wave Crash (Lv 32)
- Flail (Lv 34)
- Final Gambit (Lv 38)
- Phantom Force (Lv 40)
- Zen Headbutt (Lv 42)
- Shadow Ball (Lv 45)
- Thrash (Lv 48)
- Head Smash (Lv 53)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-basculegion-f-panel-2">
Types: Water / Ghost • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swift Swim
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM23 - Hex
- TM29 - Psychic
- TM30 - Shadow Ball
- TM42 - Facade
- TM44 - Rest
- HM03 - Surf
- HM07 - Waterfall

**Evolution Info**
Lv. after 294 Recoil Dmg, Female
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculegion-f" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">125</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-high">92</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">78</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Bitter Malice (Lv Evo)
- Shadow Force (Lv Evo)
- Tackle (Lv 1)
- Tail Whip (Lv 1)
- Water Gun (Lv 1)
- Uproar (Lv 3)
- Headbutt (Lv 5)
- Bite (Lv 7)
- Aqua Jet (Lv 9)
- Chip Away (Lv 11)
- Take Down (Lv 14)
- Crunch (Lv 17)
- Aqua Tail (Lv 20)
- Soak (Lv 23)
- Double-Edge (Lv 26)
- Scary Face (Lv 28)
- Wave Crash (Lv 32)
- Flail (Lv 34)
- Final Gambit (Lv 38)
- Phantom Force (Lv 40)
- Zen Headbutt (Lv 42)
- Shadow Ball (Lv 45)
- Thrash (Lv 48)
- Head Smash (Lv 53)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
- Sleep Talk
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
#pokemon-tabs-basculegion-f-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-basculegion-f-panel-0 { display: block; }
#pokemon-tabs-basculegion-f-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-basculegion-f-panel-1 { display: block; }
#pokemon-tabs-basculegion-f-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-basculegion-f-panel-2 { display: block; }
</style>
</details>
