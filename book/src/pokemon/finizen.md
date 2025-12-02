<details class="pokemon-card-container">
<summary>Finizen (#162)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-finizen">
<input type="radio" name="pokemon-tabs-finizen-group" id="pokemon-tabs-finizen-tab-0" checked>
<label for="pokemon-tabs-finizen-tab-0">Finizen</label>
<input type="radio" name="pokemon-tabs-finizen-group" id="pokemon-tabs-finizen-tab-1">
<label for="pokemon-tabs-finizen-tab-1">Palafin</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-finizen-panel-0">
Types: Water • Egg Groups: Field / Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Veil
- Water Veil *(Hidden)*

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
- TM13 - Ice Beam
- TM14 - Blizzard
- TM15 - Draining Kiss
- TM17 - Protect
- TM18 - Rain Dance
- TM42 - Facade
- TM44 - Rest
- TM50 - Deepwater Curse
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Davosi Straits — Surfing (60%)
- Erinys Path (East) — Fishing (20%)
- Marmaro Island — Surfing (30%)
- Myrrini Island — Surfing (10%)
- Pollen Road — Surfing (60%)
- Pythios Town — Fishing (20%)
- Pythios Town — Surfing (10%)
- Péntepetal City — Surfing (30%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="finizen" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">335</span> |

**Level-Up Moves**
- Supersonic (Lv 1)
- Water Gun (Lv 1)
- Astonish (Lv 7)
- Focus Energy (Lv 10)
- Aqua Jet (Lv 13)
- Pluck (Lv 15)
- Double Hit (Lv 17)
- Dive (Lv 21)
- Charm (Lv 23)
- Covet (Lv 26)
- Acrobatics (Lv 29)
- Encore (Lv 34)
- Aqua Tail (Lv 39)
- Mist (Lv 44)
- Hydro Pump (Lv 50)

**Egg Moves**
- Boomburst
- Bounce
- Counter
- Haze
- Tickle

**Tutor Moves**
- Body Slam
- Counter
- Endure
- Icy Wind
- Psych Up
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
<div class="pokemon-tab-panel" id="pokemon-tabs-finizen-panel-1">
Types: Water • Egg Groups: Field / Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Zero to Hero
- Iron Fist *(Hidden)*

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
- TM08 - Bulk Up
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM15 - Draining Kiss
- TM17 - Protect
- TM18 - Rain Dance
- TM42 - Facade
- TM44 - Rest
- TM50 - Deepwater Curse
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 38
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="palafin" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">72</span> |
| Sp. Atk | <span class="stat-value stat-mid">53</span> |
| Sp. Def | <span class="stat-value stat-mid">62</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">467</span> |

**Level-Up Moves**
- Flip Turn (Lv Evo)
- Jet Punch (Lv 1)
- Supersonic (Lv 1)
- Water Gun (Lv 1)
- Astonish (Lv 7)
- Focus Energy (Lv 10)
- Aqua Jet (Lv 13)
- Pluck (Lv 15)
- Double Hit (Lv 17)
- Dive (Lv 21)
- Charm (Lv 23)
- Covet (Lv 26)
- Acrobatics (Lv 29)
- Encore (Lv 34)
- Aqua Tail (Lv 39)
- Jet Punch (Lv 42)
- Deepwater Curse (Lv 44)
- Ice Hammer (Lv 47)
- Hydro Pump (Lv 50)
- Meteor Mash (Lv 53)
- Focus Punch (Lv 55)
- Wave Crash (Lv 61)

**Egg Moves**
- Boomburst
- Bounce
- Counter
- Haze
- Tickle

**Tutor Moves**
- Body Slam
- Counter
- Endure
- Ice Punch
- Icy Wind
- Psych Up
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
#pokemon-tabs-finizen-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-finizen-panel-0 { display: block; }
#pokemon-tabs-finizen-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-finizen-panel-1 { display: block; }
</style>
</details>
