<details class="pokemon-card-container">
<summary>Tyrantrum (#204)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-tyrantrum">
<input type="radio" name="pokemon-tabs-tyrantrum-group" id="pokemon-tabs-tyrantrum-tab-0">
<label for="pokemon-tabs-tyrantrum-tab-0">Tyrunt</label>
<input type="radio" name="pokemon-tabs-tyrantrum-group" id="pokemon-tabs-tyrantrum-tab-1" checked>
<label for="pokemon-tabs-tyrantrum-tab-1">Tyrantrum</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-tyrantrum-panel-0">
Types: Rock / Dragon • Egg Groups: Dragon / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Strong Jaw
- Sturdy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.25×)
- Electric (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Ice (2×)
- Fighting (2×)
- Ground (2×)
- Dragon (2×)
- Steel (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM59 - Dark Pulse
- TM60 - Dragon Dance
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Areios Hideout — Grass (Day) (2%)
- Areios Hideout — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="tyrunt" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">58</span> |
| Attack | <span class="stat-value stat-mid">89</span> |
| Defense | <span class="stat-value stat-mid">77</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-low">48</span> |
| Total | <span class="stat-value stat-mid">362</span> |

**Level-Up Moves**
- Tail Whip (Lv 1)
- Tackle (Lv 1)
- Roar (Lv 6)
- Stomp (Lv 10)
- Bide (Lv 12)
- Stealth Rock (Lv 15)
- Bite (Lv 17)
- Glare (Lv 20)
- Poison Fang (Lv 24)
- Ancient Power (Lv 26)
- Dragon Tail (Lv 30)
- Crunch (Lv 34)
- Dragon Claw (Lv 37)
- Thrash (Lv 40)
- Earthquake (Lv 44)
- Horn Drill (Lv 49)

**Egg Moves**
- Dragon Dance
- Thunder Fang
- Ice Fang
- Poison Fang
- Rock Polish
- Fire Fang
- Curse

**Tutor Moves**
- Body Slam
- Endure
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-tyrantrum-panel-1">
Types: Rock / Dragon • Egg Groups: Dragon / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Strong Jaw
- Rock Head *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.25×)
- Electric (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Ice (2×)
- Fighting (2×)
- Ground (2×)
- Dragon (2×)
- Steel (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM59 - Dark Pulse
- TM60 - Dragon Dance
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 34, Day
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="tyrantrum" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">92</span> |
| Attack | <span class="stat-value stat-high">121</span> |
| Defense | <span class="stat-value stat-high">119</span> |
| Sp. Atk | <span class="stat-value stat-mid">59</span> |
| Sp. Def | <span class="stat-value stat-mid">59</span> |
| Speed | <span class="stat-value stat-mid">71</span> |
| Total | <span class="stat-value stat-mid">521</span> |

**Level-Up Moves**
- Rock Slide (Lv Evo)
- Head Smash (Lv 1)
- Tail Whip (Lv 1)
- Tackle (Lv 1)
- Roar (Lv 6)
- Stomp (Lv 10)
- Bide (Lv 12)
- Stealth Rock (Lv 15)
- Bite (Lv 17)
- Glare (Lv 20)
- Poison Fang (Lv 24)
- Ancient Power (Lv 26)
- Dragon Tail (Lv 30)
- Crunch (Lv 34)
- Dragon Claw (Lv 37)
- Thrash (Lv 40)
- Earthquake (Lv 44)
- Horn Drill (Lv 49)
- Head Smash (Lv 54)
- Dragon Energy (Lv 60)

**Egg Moves**
- Dragon Dance
- Thunder Fang
- Ice Fang
- Poison Fang
- Rock Polish
- Fire Fang
- Curse

**Tutor Moves**
- Body Slam
- Endure
- Rock Slide
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
#pokemon-tabs-tyrantrum-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-tyrantrum-panel-0 { display: block; }
#pokemon-tabs-tyrantrum-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-tyrantrum-panel-1 { display: block; }
</style>
</details>
