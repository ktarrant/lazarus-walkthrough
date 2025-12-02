<details class="pokemon-card-container">
<summary>Skeledirge (#024)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-skeledirge">
<input type="radio" name="pokemon-tabs-skeledirge-group" id="pokemon-tabs-skeledirge-tab-0">
<label for="pokemon-tabs-skeledirge-tab-0">Fuecoco</label>
<input type="radio" name="pokemon-tabs-skeledirge-group" id="pokemon-tabs-skeledirge-tab-1">
<label for="pokemon-tabs-skeledirge-tab-1">Crocalor</label>
<input type="radio" name="pokemon-tabs-skeledirge-group" id="pokemon-tabs-skeledirge-tab-2" checked>
<label for="pokemon-tabs-skeledirge-tab-2">Skeledirge</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-skeledirge-panel-0">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Unaware
- Dry Skin *(Hidden)*

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
- TM11 - Sunny Day
- TM17 - Protect
- TM28 - Dig
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp
- TM55 - Snarl

**Encounter Locations**
- Sea of Vulcai — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="fuecoco" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">67</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">59</span> |
| Sp. Atk | <span class="stat-value stat-mid">63</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">36</span> |
| Total | <span class="stat-value stat-mid">310</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leer (Lv 1)
- Ember (Lv 1)
- Round (Lv 7)
- Bite (Lv 12)
- Incinerate (Lv 15)
- Yawn (Lv 17)
- Snarl (Lv 21)
- Roar (Lv 25)
- Flamethrower (Lv 28)
- Hyper Voice (Lv 32)
- Fire Blast (Lv 36)

**Egg Moves**
- Belch
- Curse
- Encore
- Slack Off

**Tutor Moves**
- Body Slam
- Endure
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-skeledirge-panel-1">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Unaware
- Dry Skin *(Hidden)*

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
- TM11 - Sunny Day
- TM17 - Protect
- TM28 - Dig
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp
- TM55 - Snarl

**Evolution Info**
Lv. 16
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="crocalor" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">81</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-mid">78</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">58</span> |
| Speed | <span class="stat-value stat-low">49</span> |
| Total | <span class="stat-value stat-mid">411</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leer (Lv 1)
- Ember (Lv 1)
- Lick (Lv 7)
- Round (Lv 10)
- Bite (Lv 12)
- Yawn (Lv 15)
- Incinerate (Lv 17)
- Snarl (Lv 24)
- Roar (Lv 28)
- Flamethrower (Lv 32)
- Hyper Voice (Lv 38)
- Will-O-Wisp (Lv 42)
- Fire Blast (Lv 47)

**Egg Moves**
- Belch
- Curse
- Encore
- Slack Off

**Tutor Moves**
- Body Slam
- Endure
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-skeledirge-panel-2">
Types: Fire / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Unaware
- Dry Skin *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM11 - Sunny Day
- TM17 - Protect
- TM22 - Solar Beam
- TM23 - Hex
- TM26 - Earthquake
- TM28 - Dig
- TM30 - Shadow Ball
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp
- TM55 - Snarl

**Evolution Info**
Lv. 36
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="skeledirge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">104</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-mid">66</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Torch Song (Lv Evo)
- Sing (Lv 1)
- Yawn (Lv 1)
- Ember (Lv 1)
- Tackle (Lv 1)
- Leer (Lv 1)
- Lick (Lv 7)
- Round (Lv 10)
- Scary Face (Lv 12)
- Bite (Lv 15)
- Incinerate (Lv 17)
- Snarl (Lv 24)
- Roar (Lv 28)
- Flamethrower (Lv 32)
- Shadow Ball (Lv 38)
- Hyper Voice (Lv 40)
- Will-O-Wisp (Lv 43)
- Hex (Lv 46)
- Fire Blast (Lv 50)
- Overheat (Lv 55)

**Egg Moves**
- Belch
- Curse
- Encore
- Slack Off

**Tutor Moves**
- Body Slam
- Endure
- Mud-Slap
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
#pokemon-tabs-skeledirge-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-skeledirge-panel-0 { display: block; }
#pokemon-tabs-skeledirge-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-skeledirge-panel-1 { display: block; }
#pokemon-tabs-skeledirge-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-skeledirge-panel-2 { display: block; }
</style>
</details>
