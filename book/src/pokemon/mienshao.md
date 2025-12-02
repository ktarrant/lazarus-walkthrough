<details class="pokemon-card-container">
<summary>Mienshao (#144)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-mienshao">
<input type="radio" name="pokemon-tabs-mienshao-group" id="pokemon-tabs-mienshao-tab-0">
<label for="pokemon-tabs-mienshao-tab-0">Mienfoo</label>
<input type="radio" name="pokemon-tabs-mienshao-group" id="pokemon-tabs-mienshao-tab-1" checked>
<label for="pokemon-tabs-mienshao-tab-1">Mienshao</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-mienshao-panel-0">
Types: Fighting • Egg Groups: Human-Like / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Inner Focus
- Regenerator
- Reckless *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Pythios Town — Grass (Day) (10%)
- Pythios Town — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mienfoo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">350</span> |

**Level-Up Moves**
- Pound (Lv 1)
- Meditate (Lv 5)
- Detect (Lv 9)
- Fake Out (Lv 13)
- Double Slap (Lv 17)
- Swift (Lv 21)
- U-Turn (Lv 25)
- Force Palm (Lv 29)
- Drain Punch (Lv 33)
- Poison Jab (Lv 35)
- Jump Kick (Lv 38)
- Calm Mind (Lv 41)
- Quick Guard (Lv 45)
- Fire Lash (Lv 48)
- High Jump Kick (Lv 50)
- Reversal (Lv 53)
- Aura Sphere (Lv 57)

**Egg Moves**
- Endure
- Vital Throw
- Baton Pass
- Smelling Salts
- Low Kick
- Feint
- Me First
- Knock Off
- Ally Switch

**Tutor Moves**
- Endure
- Mega Kick
- Mega Punch
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-mienshao-panel-1">
Types: Fighting • Egg Groups: Human-Like / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Inner Focus
- Regenerator
- Reckless *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 40
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mienshao" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">127</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-high">108</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Aerial Ace (Lv Evo)
- Aura Sphere (Lv 1)
- Reversal (Lv 1)
- Pound (Lv 1)
- Meditate (Lv 5)
- Detect (Lv 9)
- Fake Out (Lv 13)
- Double Slap (Lv 17)
- Swift (Lv 21)
- U-Turn (Lv 25)
- Force Palm (Lv 29)
- Drain Punch (Lv 33)
- Poison Jab (Lv 35)
- Jump Kick (Lv 38)
- Calm Mind (Lv 41)
- Quick Guard (Lv 45)
- Fire Lash (Lv 48)
- High Jump Kick (Lv 50)
- Reversal (Lv 53)
- Aura Sphere (Lv 57)

**Egg Moves**
- Endure
- Vital Throw
- Baton Pass
- Smelling Salts
- Low Kick
- Feint
- Me First
- Knock Off
- Ally Switch

**Tutor Moves**
- Double-Edge
- Endure
- Mega Kick
- Mega Punch
- Psych Up
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
#pokemon-tabs-mienshao-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-mienshao-panel-0 { display: block; }
#pokemon-tabs-mienshao-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-mienshao-panel-1 { display: block; }
</style>
</details>
