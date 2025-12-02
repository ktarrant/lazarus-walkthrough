<details class="pokemon-card-container">
<summary>Dragalge (#292)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-dragalge">
<input type="radio" name="pokemon-tabs-dragalge-group" id="pokemon-tabs-dragalge-tab-0">
<label for="pokemon-tabs-dragalge-tab-0">Skrelp</label>
<input type="radio" name="pokemon-tabs-dragalge-group" id="pokemon-tabs-dragalge-tab-1" checked>
<label for="pokemon-tabs-dragalge-tab-1">Dragalge</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-dragalge-panel-0">
Types: Poison / Water • Egg Groups: Dragon / Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Poison Point
- Poison Touch
- Adaptability *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Poison (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Electric (2×)
- Ground (2×)
- Psychic (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Kalami City — Fishing (30%)
- Kalami City — Surfing (30%)
- Kipos Town — Fishing (20%)
- Myrrini Island — Fishing (20%)
- Myrrini Island — Fishing (40%)
- Riverwalk Trail (South) — Fishing (30%)
- Riverwalk Trail (South) — Surfing (30%)
- Sea of Asteri (East) — Surfing (10%)
- Sea of Vulcai — Underwater (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="skrelp" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">320</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Smokescreen (Lv 1)
- Water Gun (Lv 1)
- Feint Attack (Lv 5)
- Tail Whip (Lv 9)
- Bubble (Lv 12)
- Acid (Lv 15)
- Camouflage (Lv 19)
- Poison Tail (Lv 23)
- Water Pulse (Lv 25)
- Double Team (Lv 28)
- Toxic (Lv 32)
- Dragon Dance (Lv 35)
- Aqua Tail (Lv 35)
- Sludge Bomb (Lv 38)
- Poison Jab (Lv 40)
- Hydro Pump (Lv 42)
- Dragon Pulse (Lv 46)

**Egg Moves**
- Toxic Spikes
- Play Rough
- Haze
- Acid Armor
- Venom Drench

**Tutor Moves**
- Acid Spray
- Endure
- Icy Wind
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-dragalge-panel-1">
Types: Poison / Dragon • Egg Groups: Dragon / Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Poison Point
- Poison Touch
- Adaptability *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.25×)
- Fighting (0.5×)
- Poison (0.5×)
- Bug (0.5×)

*Weak to*
- Ice (2×)
- Ground (2×)
- Psychic (2×)
- Dragon (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 37
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dragalge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-high">97</span> |
| Sp. Def | <span class="stat-value stat-high">123</span> |
| Speed | <span class="stat-value stat-low">39</span> |
| Total | <span class="stat-value stat-mid">494</span> |

**Level-Up Moves**
- Twister (Lv Evo)
- Dragon Tail (Lv 1)
- Tackle (Lv 1)
- Smokescreen (Lv 1)
- Water Gun (Lv 1)
- Feint Attack (Lv 5)
- Tail Whip (Lv 9)
- Bubble (Lv 12)
- Acid (Lv 15)
- Camouflage (Lv 19)
- Poison Tail (Lv 23)
- Water Pulse (Lv 25)
- Double Team (Lv 28)
- Toxic (Lv 32)
- Dragon Dance (Lv 35)
- Aqua Tail (Lv 35)
- Sludge Bomb (Lv 38)
- Poison Jab (Lv 40)
- Hydro Pump (Lv 42)
- Dragon Pulse (Lv 46)
- Dragon Tail (Lv 50)

**Egg Moves**
- Toxic Spikes
- Play Rough
- Haze
- Acid Armor
- Venom Drench

**Tutor Moves**
- Acid Spray
- Endure
- Icy Wind
- Mud-Slap
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
#pokemon-tabs-dragalge-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-dragalge-panel-0 { display: block; }
#pokemon-tabs-dragalge-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-dragalge-panel-1 { display: block; }
</style>
</details>
