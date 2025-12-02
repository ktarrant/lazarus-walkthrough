<details class="pokemon-card-container">
<summary>Whiscash (#338)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-whiscash">
<input type="radio" name="pokemon-tabs-whiscash-group" id="pokemon-tabs-whiscash-tab-0">
<label for="pokemon-tabs-whiscash-tab-0">Barboach</label>
<input type="radio" name="pokemon-tabs-whiscash-group" id="pokemon-tabs-whiscash-tab-1" checked>
<label for="pokemon-tabs-whiscash-tab-1">Whiscash</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-whiscash-panel-0">
Types: Water / Ground • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Oblivious
- Anticipation
- Hydration *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)
- Steel (0.5×)

*Weak to*
- Grass (4×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM56 - Scald
- TM60 - Dragon Dance
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Kalami City — Fishing (40%)
- Kalami City — Surfing (10%)
- Riverwalk Trail (South) — Fishing (40%)
- Riverwalk Trail (South) — Surfing (10%)
- Wanderer's Woods (North) — Fishing (30%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="barboach" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-low">48</span> |
| Defense | <span class="stat-value stat-low">43</span> |
| Sp. Atk | <span class="stat-value stat-low">46</span> |
| Sp. Def | <span class="stat-value stat-low">41</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-low">288</span> |

**Level-Up Moves**
- Mud-Slap (Lv 1)
- Mud Sport (Lv 6)
- Water Sport (Lv 6)
- Water Gun (Lv 9)
- Mud Bomb (Lv 13)
- Amnesia (Lv 15)
- Water Pulse (Lv 17)
- Magnitude (Lv 20)
- Flip Turn (Lv 23)
- Rest (Lv 25)
- Snore (Lv 25)
- Aqua Tail (Lv 28)
- Earthquake (Lv 32)
- Muddy Water (Lv 25)
- Future Sight (Lv 39)
- Fissure (Lv 44)

**Egg Moves**
- Thrash
- Whirlpool
- Spark
- Hydro Pump
- Flail
- Take Down
- Dragon Dance
- Earth Power
- Mud Shot
- Muddy Water

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-whiscash-panel-1">
Types: Water / Ground • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Oblivious
- Anticipation
- Hydration *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)
- Steel (0.5×)

*Weak to*
- Grass (4×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM56 - Scald
- TM60 - Dragon Dance
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 30

**Encounter Locations**
- Riverwalk Trail (South) — Fishing (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="whiscash" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">110</span> |
| Attack | <span class="stat-value stat-mid">78</span> |
| Defense | <span class="stat-value stat-mid">73</span> |
| Sp. Atk | <span class="stat-value stat-mid">76</span> |
| Sp. Def | <span class="stat-value stat-mid">71</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">468</span> |

**Level-Up Moves**
- Thrash (Lv Evo)
- Belch (Lv 1)
- Dragon Dance (Lv 1)
- Zen Headbutt (Lv 1)
- Tickle (Lv 1)
- Mud-Slap (Lv 1)
- Mud Sport (Lv 6)
- Water Sport (Lv 6)
- Water Gun (Lv 9)
- Mud Bomb (Lv 13)
- Amnesia (Lv 15)
- Water Pulse (Lv 17)
- Magnitude (Lv 20)
- Flip Turn (Lv 23)
- Rest (Lv 25)
- Snore (Lv 25)
- Aqua Tail (Lv 28)
- Earthquake (Lv 34)
- Muddy Water (Lv 39)
- Dragon Dance (Lv 42)
- Future Sight (Lv 45)
- Fissure (Lv 52)

**Egg Moves**
- Thrash
- Whirlpool
- Spark
- Hydro Pump
- Flail
- Take Down
- Dragon Dance
- Earth Power
- Mud Shot
- Muddy Water

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
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
#pokemon-tabs-whiscash-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-whiscash-panel-0 { display: block; }
#pokemon-tabs-whiscash-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-whiscash-panel-1 { display: block; }
</style>
</details>
