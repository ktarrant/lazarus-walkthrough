<details class="pokemon-card-container">
<summary>Rapidash (#253)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-rapidash">
<input type="radio" name="pokemon-tabs-rapidash-group" id="pokemon-tabs-rapidash-tab-0">
<label for="pokemon-tabs-rapidash-tab-0">Ponyta</label>
<input type="radio" name="pokemon-tabs-rapidash-group" id="pokemon-tabs-rapidash-tab-1" checked>
<label for="pokemon-tabs-rapidash-tab-1">Rapidash</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-rapidash-panel-0">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Run Away
- Flash Fire
- Flame Body *(Hidden)*

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
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM22 - Solar Beam
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM51 - Will-O-Wisp
- HM04 - Strength

**Encounter Locations**
- Lastlight Road — Grass (Day) (10%)
- Riverwalk Trail (West) — Grass (Day) (20%)
- Sofos City — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ponyta" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">410</span> |

**Level-Up Moves**
- Growl (Lv 1)
- Tackle (Lv 1)
- Tail Whip (Lv 4)
- Ember (Lv 9)
- Flame Wheel (Lv 13)
- Stomp (Lv 17)
- Flame Charge (Lv 21)
- Fire Spin (Lv 25)
- Take Down (Lv 29)
- Inferno (Lv 33)
- Agility (Lv 37)
- Fire Blast (Lv 41)
- Bounce (Lv 45)
- Flare Blitz (Lv 49)

**Egg Moves**
- Flame Wheel
- Thrash
- Double Kick
- Hypnosis
- Charm
- Double-Edge
- Horn Drill
- Morning Sun
- Low Kick
- Captivate
- Ally Switch
- High Horsepower

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-rapidash-panel-1">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Run Away
- Flash Fire
- Flame Body *(Hidden)*

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
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM20 - Poison Jab
- TM22 - Solar Beam
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM51 - Will-O-Wisp
- HM04 - Strength

**Evolution Info**
Lv. 30

**Encounter Locations**
- Port Pello — Grass (Day) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="rapidash" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">115</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Megahorn (Lv Evo)
- Fury Attack (Lv 1)
- Poison Jab (Lv 1)
- Growl (Lv 1)
- Quick Attack (Lv 1)
- Tail Whip (Lv 4)
- Ember (Lv 9)
- Flame Wheel (Lv 13)
- Stomp (Lv 17)
- Flame Charge (Lv 21)
- Fire Spin (Lv 25)
- Take Down (Lv 29)
- Inferno (Lv 33)
- Agility (Lv 37)
- Fire Blast (Lv 41)
- Bounce (Lv 45)
- Flare Blitz (Lv 49)

**Egg Moves**
- Flame Wheel
- Thrash
- Double Kick
- Hypnosis
- Charm
- Double-Edge
- Horn Drill
- Morning Sun
- Low Kick
- Captivate
- Ally Switch
- High Horsepower

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
- Snore
- Swagger
- Swift
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
#pokemon-tabs-rapidash-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-rapidash-panel-0 { display: block; }
#pokemon-tabs-rapidash-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-rapidash-panel-1 { display: block; }
</style>
</details>
