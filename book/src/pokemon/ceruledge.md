<details class="pokemon-card-container">
<summary>Ceruledge (#304)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-ceruledge">
<input type="radio" name="pokemon-tabs-ceruledge-group" id="pokemon-tabs-ceruledge-tab-0">
<label for="pokemon-tabs-ceruledge-tab-0">Charcadet</label>
<input type="radio" name="pokemon-tabs-ceruledge-group" id="pokemon-tabs-ceruledge-tab-1">
<label for="pokemon-tabs-ceruledge-tab-1">Armarouge</label>
<input type="radio" name="pokemon-tabs-ceruledge-group" id="pokemon-tabs-ceruledge-tab-2" checked>
<label for="pokemon-tabs-ceruledge-tab-2">Ceruledge</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-ceruledge-panel-0">
Types: Fire • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Flame Body
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- TM11 - Sunny Day
- TM17 - Protect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp

**Encounter Locations**
- Sea of Asteri (East) — Grass (Day) (5%)
- Sea of Vulcai — Grass (Day) (10%)
- Sea of Vulcai — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="charcadet" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-low">255</span> |

**Level-Up Moves**
- Leer (Lv 1)
- Ember (Lv 1)
- Astonish (Lv 1)
- Clear Smog (Lv 8)
- Fire Spin (Lv 12)
- Will-O-Wisp (Lv 16)
- Night Shade (Lv 20)
- Flame Charge (Lv 24)
- Incinerate (Lv 28)
- Lava Plume (Lv 32)

**Egg Moves**
- Destiny Bond
- Disable
- Spite

**Tutor Moves**
- Endure
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ceruledge-panel-1">
Types: Fire / Psychic • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Mega Launcher
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Psychic (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM11 - Sunny Day
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM22 - Solar Beam
- TM29 - Psychic
- TM30 - Shadow Ball
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp
- TM59 - Dark Pulse

**Evolution Info**
Auspicious Armor

**Encounter Locations**
- Lastlight Road — Grass (Day) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="armarouge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- Psyshock (Lv Evo)
- Leer (Lv 1)
- Ember (Lv 1)
- Mystical Fire (Lv 1)
- Astonish (Lv 1)
- Wide Guard (Lv 1)
- Clear Smog (Lv 8)
- Fire Spin (Lv 12)
- Will-O-Wisp (Lv 16)
- Night Shade (Lv 20)
- Flame Charge (Lv 24)
- Incinerate (Lv 28)
- Lava Plume (Lv 32)
- Aura Sphere (Lv 34)
- Calm Mind (Lv 37)
- Ally Switch (Lv 39)
- Flamethrower (Lv 41)
- Expanding Force (Lv 45)
- Armor Cannon (Lv 50)

**Egg Moves**
- Destiny Bond
- Disable
- Spite

**Tutor Moves**
- Acid Spray
- Endure
- Psych Up
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ceruledge-panel-2">
Types: Fire / Ghost • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Sharpness
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM20 - Poison Jab
- TM23 - Hex
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp

**Evolution Info**
Malicious Armor

**Encounter Locations**
- Lastlight Road — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ceruledge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-mid">85</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- Shadow Claw (Lv Evo)
- Night Slash (Lv 1)
- Shadow Sneak (Lv 1)
- Quick Guard (Lv 1)
- Solar Blade (Lv 1)
- Leer (Lv 1)
- Ember (Lv 1)
- Astonish (Lv 1)
- Clear Smog (Lv 8)
- Fire Spin (Lv 12)
- Will-O-Wisp (Lv 16)
- Night Shade (Lv 20)
- Flame Charge (Lv 24)
- Incinerate (Lv 28)
- Lava Plume (Lv 32)
- Sacred Sword (Lv 34)
- Swords Dance (Lv 37)
- Ally Switch (Lv 39)
- Bitter Blade (Lv 41)
- Psycho Cut (Lv 45)
- Flare Blitz (Lv 50)

**Egg Moves**
- Destiny Bond
- Disable
- Spite

**Tutor Moves**
- Endure
- Psych Up
- Sleep Talk
- Swords Dance
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
#pokemon-tabs-ceruledge-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-ceruledge-panel-0 { display: block; }
#pokemon-tabs-ceruledge-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-ceruledge-panel-1 { display: block; }
#pokemon-tabs-ceruledge-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-ceruledge-panel-2 { display: block; }
</style>
</details>
