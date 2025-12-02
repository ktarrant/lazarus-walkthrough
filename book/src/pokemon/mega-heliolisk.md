<details class="pokemon-card-container">
<summary>Mega Heliolisk (#319)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-mega-heliolisk">
<input type="radio" name="pokemon-tabs-mega-heliolisk-group" id="pokemon-tabs-mega-heliolisk-tab-0">
<label for="pokemon-tabs-mega-heliolisk-tab-0">Heliolisk</label>
<input type="radio" name="pokemon-tabs-mega-heliolisk-group" id="pokemon-tabs-mega-heliolisk-tab-1">
<label for="pokemon-tabs-mega-heliolisk-tab-1">Helioptile</label>
<input type="radio" name="pokemon-tabs-mega-heliolisk-group" id="pokemon-tabs-mega-heliolisk-tab-2" checked>
<label for="pokemon-tabs-mega-heliolisk-tab-2">Mega Heliolisk</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-heliolisk-panel-0">
Types: Electric / Normal • Egg Groups: Dragon / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Dry Skin
- Sand Veil
- Solar Power *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Ghost (0×)
- Steel (0.5×)

*Weak to*
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM05 - Flash

**Evolution Info**
Sun Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="heliolisk" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">62</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-mid">52</span> |
| Sp. Atk | <span class="stat-value stat-high">119</span> |
| Sp. Def | <span class="stat-value stat-high">94</span> |
| Speed | <span class="stat-value stat-high">109</span> |
| Total | <span class="stat-value stat-mid">491</span> |

**Level-Up Moves**
- Snarl (Lv Evo)
- Eerie Impulse (Lv Evo)
- Pound (Lv 1)
- Tail Whip (Lv 1)
- Thunder Shock (Lv 6)
- Charge (Lv 11)
- Mud-Slap (Lv 13)
- Quick Attack (Lv 17)
- Dig (Lv 20)
- Uproar (Lv 22)
- Parabolic Charge (Lv 25)
- Nuzzle (Lv 27)
- Flame Burst (Lv 29)
- Thunder Wave (Lv 31)
- Bulldoze (Lv 34)
- Volt Switch (Lv 36)
- Discharge (Lv 38)
- Flamethrower (Lv 41)
- Electrify (Lv 43)
- Thunderbolt (Lv 46)
- Hyper Voice (Lv 50)

**Egg Moves**
- Agility
- Glare
- Camouflage
- Electric Terrain

**Tutor Moves**
- Endure
- Fire Punch
- Mega Kick
- Mega Punch
- Mud-Slap
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
- Thunder Punch
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
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-heliolisk-panel-1">
Types: Electric / Normal • Egg Groups: Dragon / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Dry Skin
- Sand Veil
- Solar Power *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Ghost (0×)
- Steel (0.5×)

*Weak to*
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM05 - Flash

**Encounter Locations**
- Fresco Isles — Grass (Day) (5%)
- Lastlight Road — Grass (Day) (10%)
- Lastlight Road — Grass (Night) (10%)
- Sofos City — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="helioptile" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">44</span> |
| Attack | <span class="stat-value stat-low">38</span> |
| Defense | <span class="stat-value stat-low">33</span> |
| Sp. Atk | <span class="stat-value stat-mid">61</span> |
| Sp. Def | <span class="stat-value stat-low">43</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-low">289</span> |

**Level-Up Moves**
- Pound (Lv 1)
- Tail Whip (Lv 1)
- Thunder Shock (Lv 6)
- Charge (Lv 11)
- Mud-Slap (Lv 13)
- Quick Attack (Lv 17)
- Dig (Lv 20)
- Uproar (Lv 22)
- Parabolic Charge (Lv 25)
- Nuzzle (Lv 27)
- Flame Burst (Lv 29)
- Thunder Wave (Lv 31)
- Bulldoze (Lv 34)
- Volt Switch (Lv 36)
- Discharge (Lv 38)
- Flamethrower (Lv 41)
- Electrify (Lv 43)
- Thunderbolt (Lv 46)
- Hyper Voice (Lv 50)

**Egg Moves**
- Agility
- Glare
- Camouflage
- Electric Terrain

**Tutor Moves**
- Endure
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-mega-heliolisk-panel-2">
Types: Electric / Normal • Egg Groups: Dragon / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Drought

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Ghost (0×)
- Steel (0.5×)

*Weak to*
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM05 - Flash

**Evolution Info**
Heliolite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-heliolisk" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">62</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">72</span> |
| Sp. Atk | <span class="stat-value stat-high">159</span> |
| Sp. Def | <span class="stat-value stat-high">104</span> |
| Speed | <span class="stat-value stat-high">119</span> |
| Total | <span class="stat-value stat-high">591</span> |

**Level-Up Moves**
- Snarl (Lv Evo)
- Eerie Impulse (Lv Evo)
- Pound (Lv 1)
- Tail Whip (Lv 1)
- Thunder Shock (Lv 6)
- Charge (Lv 11)
- Mud-Slap (Lv 13)
- Quick Attack (Lv 17)
- Dig (Lv 20)
- Uproar (Lv 22)
- Parabolic Charge (Lv 25)
- Nuzzle (Lv 27)
- Flame Burst (Lv 29)
- Thunder Wave (Lv 31)
- Bulldoze (Lv 34)
- Volt Switch (Lv 36)
- Discharge (Lv 38)
- Flamethrower (Lv 41)
- Electrify (Lv 43)
- Thunderbolt (Lv 46)
- Hyper Voice (Lv 50)

**Egg Moves**
- Agility
- Glare
- Camouflage
- Electric Terrain

**Tutor Moves**
- Endure
- Fire Punch
- Mega Kick
- Mega Punch
- Mud-Slap
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
- Thunder Punch
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
#pokemon-tabs-mega-heliolisk-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-heliolisk-panel-0 { display: block; }
#pokemon-tabs-mega-heliolisk-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-heliolisk-panel-1 { display: block; }
#pokemon-tabs-mega-heliolisk-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-mega-heliolisk-panel-2 { display: block; }
</style>
</details>
