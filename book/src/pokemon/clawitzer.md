<details class="pokemon-card-container">
<summary>Clawitzer (#263)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-clawitzer">
<input type="radio" name="pokemon-tabs-clawitzer-group" id="pokemon-tabs-clawitzer-tab-0">
<label for="pokemon-tabs-clawitzer-tab-0">Clauncher</label>
<input type="radio" name="pokemon-tabs-clawitzer-group" id="pokemon-tabs-clawitzer-tab-1" checked>
<label for="pokemon-tabs-clawitzer-tab-1">Clawitzer</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-clawitzer-panel-0">
Types: Water • Egg Groups: Water 3 / Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Mega Launcher

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
- TM06 - Toxic
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Kalami City — Fishing (60%)
- Kalami City — Surfing (60%)
- Marmaro Island — Fishing (20%)
- Marmaro Island — Fishing (40%)
- Pythios Cemetery — Surfing (10%)
- Riverwalk Trail (South) — Fishing (60%)
- Riverwalk Trail (South) — Surfing (60%)
- Riverwalk Trail (West) — Fishing (60%)
- Riverwalk Trail (West) — Surfing (60%)
- Sofos City — Fishing (20%)
- Sofos City — Fishing (40%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="clauncher" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">53</span> |
| Defense | <span class="stat-value stat-mid">62</span> |
| Sp. Atk | <span class="stat-value stat-mid">58</span> |
| Sp. Def | <span class="stat-value stat-mid">63</span> |
| Speed | <span class="stat-value stat-low">44</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- Splash (Lv 1)
- Water Gun (Lv 1)
- Water Sport (Lv 7)
- Vise Grip (Lv 9)
- Bubble (Lv 12)
- Flail (Lv 16)
- Bubble Beam (Lv 20)
- Bug Bite (Lv 23)
- Swords Dance (Lv 25)
- Crabhammer (Lv 30)
- Water Pulse (Lv 32)
- Bug Buzz (Lv 35)
- Smack Down (Lv 39)
- Aqua Jet (Lv 43)
- Octazooka (Lv 50)

**Egg Moves**
- Aqua Jet
- Entrainment
- Endure
- Crabhammer
- Helping Hand

**Tutor Moves**
- Endure
- Icy Wind
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-clawitzer-panel-1">
Types: Water • Egg Groups: Water 3 / Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Mega Launcher

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
- TM06 - Toxic
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM30 - Shadow Ball
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- TM59 - Dark Pulse
- HM01 - Cut
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 37

**Encounter Locations**
- Riverwalk Trail (West) — Fishing (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="clawitzer" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">76</span> |
| Attack | <span class="stat-value stat-mid">73</span> |
| Defense | <span class="stat-value stat-mid">88</span> |
| Sp. Atk | <span class="stat-value stat-high">120</span> |
| Sp. Def | <span class="stat-value stat-mid">89</span> |
| Speed | <span class="stat-value stat-mid">54</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Aura Sphere (Lv Evo)
- Dark Pulse (Lv 1)
- Dragon Pulse (Lv 1)
- Heal Pulse (Lv 1)
- Splash (Lv 1)
- Water Gun (Lv 1)
- Water Sport (Lv 7)
- Vise Grip (Lv 9)
- Bubble (Lv 12)
- Flail (Lv 16)
- Bubble Beam (Lv 20)
- Bug Bite (Lv 23)
- Swords Dance (Lv 25)
- Crabhammer (Lv 30)
- Water Pulse (Lv 32)
- Bug Buzz (Lv 35)
- Smack Down (Lv 39)
- Aqua Jet (Lv 43)
- Dragon Pulse (Lv 45)
- Octazooka (Lv 50)

**Egg Moves**
- Aqua Jet
- Entrainment
- Endure
- Crabhammer
- Helping Hand

**Tutor Moves**
- Body Slam
- Endure
- Icy Wind
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
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
#pokemon-tabs-clawitzer-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-clawitzer-panel-0 { display: block; }
#pokemon-tabs-clawitzer-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-clawitzer-panel-1 { display: block; }
</style>
</details>
