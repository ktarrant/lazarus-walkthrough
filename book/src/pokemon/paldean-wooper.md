<details class="pokemon-card-container">
<summary>Paldean Wooper (#134)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-paldean-wooper">
<input type="radio" name="pokemon-tabs-paldean-wooper-group" id="pokemon-tabs-paldean-wooper-tab-0" checked>
<label for="pokemon-tabs-paldean-wooper-tab-0">Paldean Wooper</label>
<input type="radio" name="pokemon-tabs-paldean-wooper-group" id="pokemon-tabs-paldean-wooper-tab-1">
<label for="pokemon-tabs-paldean-wooper-tab-1">Clodsire</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-paldean-wooper-panel-0">
Types: Poison / Ground • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Poison Point
- Water Absorb
- Unaware *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Fighting (0.5×)
- Poison (0.25×)
- Bug (0.5×)
- Rock (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ice (2×)
- Ground (2×)
- Psychic (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- HM03 - Surf
- HM07 - Waterfall

**Encounter Locations**
- Fresco Isles — Grass (Night) (5%)
- Riverwalk Trail (South) — Grass (Day) (5%)
- Riverwalk Trail (South) — Grass (Night) (5%)
- Wanderer's Woods (North) — Grass (Night) (10%)
- Wanderer's Woods (South) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="paldean-wooper" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-low">25</span> |
| Sp. Def | <span class="stat-value stat-low">25</span> |
| Speed | <span class="stat-value stat-low">15</span> |
| Total | <span class="stat-value stat-low">210</span> |

**Level-Up Moves**
- Tail Whip (Lv 1)
- Mud Shot (Lv 1)
- Tackle (Lv 4)
- Poison Tail (Lv 8)
- Toxic Spikes (Lv 12)
- Slam (Lv 16)
- Yawn (Lv 21)
- Poison Jab (Lv 24)
- Sludge Wave (Lv 28)
- Amnesia (Lv 32)
- Toxic (Lv 36)
- Earthquake (Lv 40)

**Egg Moves**
- Acid Spray
- After You
- Ancient Power
- Counter
- Curse
- Double Kick
- Haze
- Mist
- Recover
- Spit Up
- Stockpile
- Swallow

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Double-Edge
- Endure
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-paldean-wooper-panel-1">
Types: Poison / Ground • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Poison Point
- Water Absorb
- Unaware *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Fighting (0.5×)
- Poison (0.25×)
- Bug (0.5×)
- Rock (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ice (2×)
- Ground (2×)
- Psychic (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- HM03 - Surf
- HM07 - Waterfall

**Evolution Info**
Lv. 20

**Encounter Locations**
- Palati City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="clodsire" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">130</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-mid">430</span> |

**Level-Up Moves**
- Amnesia (Lv Evo)
- Tail Whip (Lv 1)
- Poison Sting (Lv 1)
- Toxic Spikes (Lv 4)
- Mud Shot (Lv 8)
- Poison Tail (Lv 12)
- Slam (Lv 16)
- Yawn (Lv 21)
- Poison Jab (Lv 24)
- Sludge Wave (Lv 30)
- Megahorn (Lv 36)
- Toxic (Lv 40)
- Earthquake (Lv 44)
- Shore Up (Lv 48)

**Egg Moves**
- Acid Spray
- After You
- Ancient Power
- Counter
- Curse
- Double Kick
- Haze
- Mist
- Recover
- Spit Up
- Stockpile
- Swallow

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Double-Edge
- Endure
- Mud-Slap
- Rock Slide
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
</div>
</div>
<style>
#pokemon-tabs-paldean-wooper-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-paldean-wooper-panel-0 { display: block; }
#pokemon-tabs-paldean-wooper-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-paldean-wooper-panel-1 { display: block; }
</style>
</details>
