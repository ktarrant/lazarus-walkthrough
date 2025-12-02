<details class="pokemon-card-container">
<summary>Chinchou (#305)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-chinchou">
<input type="radio" name="pokemon-tabs-chinchou-group" id="pokemon-tabs-chinchou-tab-0" checked>
<label for="pokemon-tabs-chinchou-tab-0">Chinchou</label>
<input type="radio" name="pokemon-tabs-chinchou-group" id="pokemon-tabs-chinchou-tab-1">
<label for="pokemon-tabs-chinchou-tab-1">Lanturn</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-chinchou-panel-0">
Types: Water / Electric • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Illuminate
- Water Absorb *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Flying (0.5×)
- Steel (0.25×)

*Weak to*
- Grass (2×)
- Ground (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam
- TM56 - Scald
- TM58 - Thunder Wave
- HM03 - Surf
- HM05 - Flash
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
Deep Sea Scale

**Encounter Locations**
- Fresco Isles — Fishing (15%)
- Marmaro Island — Surfing (60%)
- Myrrini Island — Fishing (15%)
- Sea of Asteri (East) — Fishing (15%)
- Sea of Asteri (West) — Fishing (15%)
- Sea of Vulcai — Fishing (15%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="chinchou" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-low">38</span> |
| Defense | <span class="stat-value stat-low">43</span> |
| Sp. Atk | <span class="stat-value stat-mid">61</span> |
| Sp. Def | <span class="stat-value stat-mid">56</span> |
| Speed | <span class="stat-value stat-mid">57</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- Bubble (Lv 1)
- Supersonic (Lv 1)
- Thunder Wave (Lv 6)
- Electro Ball (Lv 9)
- Water Gun (Lv 12)
- Confuse Ray (Lv 17)
- Bubble Beam (Lv 20)
- Spark (Lv 23)
- Signal Beam (Lv 28)
- Flail (Lv 31)
- Discharge (Lv 34)
- Yawn (Lv 37)
- Surf (Lv 39)
- Aqua Ring (Lv 42)
- Hydro Pump (Lv 45)
- Ion Deluge (Lv 47)
- Glitzy Glow (Lv 50)

**Egg Moves**
- Flail
- Screech
- Amnesia
- Psybeam
- Whirlpool
- Agility
- Mist
- Shock Wave
- Brine
- Water Pulse
- Soak

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
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
<div class="pokemon-tab-panel" id="pokemon-tabs-chinchou-panel-1">
Types: Water / Electric • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Volt Absorb
- Illuminate
- Water Absorb *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Flying (0.5×)
- Steel (0.25×)

*Weak to*
- Grass (2×)
- Ground (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam
- TM56 - Scald
- TM58 - Thunder Wave
- HM03 - Surf
- HM05 - Flash
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
Deep Sea Scale

**Evolution Info**
Lv. 27

**Encounter Locations**
- Myrrini Island — Fishing (5%)
- Sea of Asteri (East) — Fishing (5%)
- Sea of Asteri (West) — Fishing (5%)
- Sea of Vulcai — Fishing (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="lanturn" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">125</span> |
| Attack | <span class="stat-value stat-mid">58</span> |
| Defense | <span class="stat-value stat-mid">63</span> |
| Sp. Atk | <span class="stat-value stat-mid">81</span> |
| Sp. Def | <span class="stat-value stat-mid">76</span> |
| Speed | <span class="stat-value stat-mid">62</span> |
| Total | <span class="stat-value stat-mid">465</span> |

**Level-Up Moves**
- Tail Glow (Lv Evo)
- Stockpile (Lv 1)
- Swallow (Lv 1)
- Spit Up (Lv 1)
- Charge (Lv 1)
- Eerie Impulse (Lv 1)
- Spotlight (Lv 1)
- Bubble (Lv 1)
- Supersonic (Lv 1)
- Thunder Wave (Lv 6)
- Electro Ball (Lv 9)
- Water Gun (Lv 12)
- Confuse Ray (Lv 17)
- Bubble Beam (Lv 20)
- Spark (Lv 23)
- Signal Beam (Lv 28)
- Flail (Lv 31)
- Discharge (Lv 34)
- Yawn (Lv 37)
- Surf (Lv 39)
- Aqua Ring (Lv 42)
- Hydro Pump (Lv 45)
- Ion Deluge (Lv 47)
- Glitzy Glow (Lv 50)

**Egg Moves**
- Flail
- Screech
- Amnesia
- Psybeam
- Whirlpool
- Agility
- Mist
- Shock Wave
- Brine
- Water Pulse
- Soak

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
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
#pokemon-tabs-chinchou-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-chinchou-panel-0 { display: block; }
#pokemon-tabs-chinchou-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-chinchou-panel-1 { display: block; }
</style>
</details>
