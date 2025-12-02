<details class="pokemon-card-container">
<summary>Omanyte (#171)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-omanyte">
<input type="radio" name="pokemon-tabs-omanyte-group" id="pokemon-tabs-omanyte-tab-0" checked>
<label for="pokemon-tabs-omanyte-tab-0">Omanyte</label>
<input type="radio" name="pokemon-tabs-omanyte-group" id="pokemon-tabs-omanyte-tab-1">
<label for="pokemon-tabs-omanyte-tab-1">Omastar</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-omanyte-panel-0">
Types: Rock / Water • Egg Groups: Water 1 / Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Skill Link
- Shell Armor
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.25×)
- Ice (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Electric (2×)
- Grass (4×)
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- HM03 - Surf
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Erinys Path (West) — Grass (Day) (10%)
- Erinys Path (West) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="omanyte" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">35</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-mid">355</span> |

**Level-Up Moves**
- Constrict (Lv 1)
- Withdraw (Lv 1)
- Bite (Lv 7)
- Water Gun (Lv 10)
- Rollout (Lv 16)
- Leer (Lv 19)
- Mud Shot (Lv 25)
- Brine (Lv 28)
- Icicle Spear (Lv 31)
- Protect (Lv 34)
- Ancient Power (Lv 37)
- Tickle (Lv 43)
- Rock Blast (Lv 45)
- Shell Smash (Lv 50)
- Hydro Pump (Lv 55)

**Egg Moves**
- Bubble Beam
- Aurora Beam
- Slam
- Supersonic
- Haze
- Spikes
- Knock Off
- Wring Out
- Toxic Spikes
- Muddy Water
- Bide
- Water Pulse
- Whirlpool
- Reflect Type

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-omanyte-panel-1">
Types: Rock / Water • Egg Groups: Water 1 / Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Skill Link
- Shell Armor
- Weak Armor *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.25×)
- Ice (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Electric (2×)
- Grass (4×)
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- HM03 - Surf
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 40
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="omastar" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-high">125</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">495</span> |

**Level-Up Moves**
- Spike Cannon (Lv Evo)
- Constrict (Lv 1)
- Withdraw (Lv 1)
- Bite (Lv 7)
- Water Gun (Lv 10)
- Rollout (Lv 16)
- Leer (Lv 19)
- Mud Shot (Lv 25)
- Brine (Lv 28)
- Icicle Spear (Lv 31)
- Protect (Lv 34)
- Ancient Power (Lv 37)
- Tickle (Lv 43)
- Rock Blast (Lv 45)
- Shell Smash (Lv 50)
- Hydro Pump (Lv 55)

**Egg Moves**
- Bubble Beam
- Aurora Beam
- Slam
- Supersonic
- Haze
- Spikes
- Knock Off
- Wring Out
- Toxic Spikes
- Muddy Water
- Bide
- Water Pulse
- Whirlpool
- Reflect Type

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Rock Slide
- Rollout
- Seismic Toss
- Sleep Talk
- Snore
- Swagger
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
#pokemon-tabs-omanyte-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-omanyte-panel-0 { display: block; }
#pokemon-tabs-omanyte-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-omanyte-panel-1 { display: block; }
</style>
</details>
