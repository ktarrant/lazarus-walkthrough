<details class="pokemon-card-container">
<summary>Ninetales (#148)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-ninetales">
<input type="radio" name="pokemon-tabs-ninetales-group" id="pokemon-tabs-ninetales-tab-0">
<label for="pokemon-tabs-ninetales-tab-0">Vulpix</label>
<input type="radio" name="pokemon-tabs-ninetales-group" id="pokemon-tabs-ninetales-tab-1" checked>
<label for="pokemon-tabs-ninetales-tab-1">Ninetales</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-ninetales-panel-0">
Types: Fire / Fairy • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Drought *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Bug (0.25×)
- Dragon (0×)
- Dark (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Poison (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM23 - Hex
- TM28 - Dig
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM51 - Will-O-Wisp
- TM55 - Snarl
- TM59 - Dark Pulse

**Held Item**
Charcoal

**Encounter Locations**
- Jusmail Town — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="vulpix" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">38</span> |
| Attack | <span class="stat-value stat-low">41</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">309</span> |

**Level-Up Moves**
- Ember (Lv 1)
- Tail Whip (Lv 4)
- Roar (Lv 7)
- Baby-Doll Eyes (Lv 9)
- Quick Attack (Lv 10)
- Confuse Ray (Lv 12)
- Fire Spin (Lv 15)
- Disarming Voice (Lv 18)
- Will-O-Wisp (Lv 20)
- Feint Attack (Lv 23)
- Hex (Lv 26)
- Flame Burst (Lv 28)
- Extrasensory (Lv 31)
- Dazzling Gleam (Lv 33)
- Safeguard (Lv 34)
- Flamethrower (Lv 36)
- Imprison (Lv 39)
- Fire Blast (Lv 42)
- Grudge (Lv 44)
- Moonblast (Lv 46)
- Captivate (Lv 47)
- Inferno (Lv 50)

**Egg Moves**
- Feint Attack
- Hypnosis
- Flail
- Spite
- Disable
- Howl
- Heat Wave
- Flare Blitz
- Extrasensory
- Power Swap
- Secret Power
- Hex
- Tail Slap
- Captivate

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Psych Up
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ninetales-panel-1">
Types: Fire / Fairy • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- Drought *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Bug (0.25×)
- Dragon (0×)
- Dark (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Poison (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM22 - Solar Beam
- TM23 - Hex
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM51 - Will-O-Wisp
- TM55 - Snarl
- TM59 - Dark Pulse

**Held Item**
Charcoal

**Evolution Info**
Fire Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ninetales" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">73</span> |
| Attack | <span class="stat-value stat-mid">76</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">101</span> |
| Sp. Def | <span class="stat-value stat-high">96</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">521</span> |

**Level-Up Moves**
- Ember (Lv 1)
- Tail Whip (Lv 4)
- Roar (Lv 7)
- Baby-Doll Eyes (Lv 9)
- Quick Attack (Lv 10)
- Confuse Ray (Lv 12)
- Fire Spin (Lv 15)
- Disarming Voice (Lv 18)
- Will-O-Wisp (Lv 20)
- Feint Attack (Lv 23)
- Hex (Lv 26)
- Flame Burst (Lv 28)
- Extrasensory (Lv 31)
- Dazzling Gleam (Lv 33)
- Safeguard (Lv 34)
- Flamethrower (Lv 36)
- Imprison (Lv 39)
- Fire Blast (Lv 42)
- Grudge (Lv 44)
- Moonblast (Lv 46)
- Captivate (Lv 47)
- Inferno (Lv 50)

**Egg Moves**
- Feint Attack
- Hypnosis
- Flail
- Spite
- Disable
- Howl
- Heat Wave
- Flare Blitz
- Extrasensory
- Power Swap
- Secret Power
- Hex
- Tail Slap
- Captivate

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Psych Up
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
#pokemon-tabs-ninetales-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-ninetales-panel-0 { display: block; }
#pokemon-tabs-ninetales-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-ninetales-panel-1 { display: block; }
</style>
</details>
