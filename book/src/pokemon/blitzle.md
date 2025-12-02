<details class="pokemon-card-container">
<summary>Blitzle (#139)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-blitzle">
<input type="radio" name="pokemon-tabs-blitzle-group" id="pokemon-tabs-blitzle-tab-0" checked>
<label for="pokemon-tabs-blitzle-tab-0">Blitzle</label>
<input type="radio" name="pokemon-tabs-blitzle-group" id="pokemon-tabs-blitzle-tab-1">
<label for="pokemon-tabs-blitzle-tab-1">Zebstrika</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-blitzle-panel-0">
Types: Electric • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Lightning Rod
- Motor Drive
- Sap Sipper *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM58 - Thunder Wave
- HM05 - Flash

**Encounter Locations**
- Wanderer's Woods (North) — Grass (Day) (2%)
- Wanderer's Woods (North) — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="blitzle" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">32</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">37</span> |
| Speed | <span class="stat-value stat-mid">76</span> |
| Total | <span class="stat-value stat-low">305</span> |

**Level-Up Moves**
- Quick Attack (Lv 1)
- Tail Whip (Lv 4)
- Charge (Lv 8)
- Shock Wave (Lv 11)
- Thunder Wave (Lv 15)
- Flame Charge (Lv 18)
- Pursuit (Lv 22)
- Spark (Lv 24)
- Stomp (Lv 27)
- Blaze Kick (Lv 31)
- Discharge (Lv 34)
- Agility (Lv 36)
- Wild Charge (Lv 39)
- Trop Kick (Lv 41)
- Stomping Tantrum (Lv 44)
- Thunderous Kick (Lv 48)
- Thrash (Lv 52)

**Egg Moves**
- Me First
- Take Down
- Sand Attack
- Double Kick
- Screech
- Rage
- Endure
- Double-Edge
- Shock Wave
- Snatch
- Feint

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
<div class="pokemon-tab-panel" id="pokemon-tabs-blitzle-panel-1">
Types: Electric • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Lightning Rod
- Motor Drive
- Sap Sipper *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM58 - Thunder Wave
- HM05 - Flash
- HM06 - Rock Smash

**Evolution Info**
Lv. 27
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="zebstrika" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-mid">63</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">63</span> |
| Speed | <span class="stat-value stat-high">116</span> |
| Total | <span class="stat-value stat-mid">517</span> |

**Level-Up Moves**
- Quick Attack (Lv 1)
- Tail Whip (Lv 4)
- Charge (Lv 8)
- Shock Wave (Lv 11)
- Thunder Wave (Lv 15)
- Flame Charge (Lv 18)
- Pursuit (Lv 22)
- Spark (Lv 24)
- Stomp (Lv 27)
- Blaze Kick (Lv 31)
- Discharge (Lv 34)
- Agility (Lv 36)
- Wild Charge (Lv 39)
- Trop Kick (Lv 41)
- Stomping Tantrum (Lv 44)
- Thunderous Kick (Lv 48)
- Thrash (Lv 52)
- Ion Deluge (Lv 55)

**Egg Moves**
- Me First
- Take Down
- Sand Attack
- Double Kick
- Screech
- Rage
- Endure
- Double-Edge
- Shock Wave
- Snatch
- Feint

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
</div>
</div>
<style>
#pokemon-tabs-blitzle-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-blitzle-panel-0 { display: block; }
#pokemon-tabs-blitzle-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-blitzle-panel-1 { display: block; }
</style>
</details>
