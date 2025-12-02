<details class="pokemon-card-container">
<summary>Bewear (#268)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-bewear">
<input type="radio" name="pokemon-tabs-bewear-group" id="pokemon-tabs-bewear-tab-0">
<label for="pokemon-tabs-bewear-tab-0">Stufful</label>
<input type="radio" name="pokemon-tabs-bewear-group" id="pokemon-tabs-bewear-tab-1" checked>
<label for="pokemon-tabs-bewear-tab-1">Bewear</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-bewear-panel-0">
Types: Normal / Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Fluffy
- Reckless
- Cute Charm *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Ghost (0×)
- Dark (0.5×)

*Weak to*
- Fighting (2×)
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM04 - Strength

**Encounter Locations**
- Sea of Asteri (East) — Grass (Day) (10%)
- Sea of Asteri (West) — Grass (Day) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="stufful" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-mid">340</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leer (Lv 1)
- Bide (Lv 5)
- Baby-Doll Eyes (Lv 10)
- Brutal Swing (Lv 14)
- Flail (Lv 19)
- Payback (Lv 23)
- Take Down (Lv 28)
- Hammer Arm (Lv 32)
- Slack Off (Lv 35)
- Thrash (Lv 37)
- Pain Split (Lv 41)
- Double-Edge (Lv 46)
- Superpower (Lv 50)

**Egg Moves**
- Ice Punch
- Thunder Punch
- Force Palm
- Endure
- Wide Guard
- Mega Kick
- Stomping Tantrum

**Tutor Moves**
- Defense Curl
- Double-Edge
- Endure
- Ice Punch
- Mega Kick
- Mega Punch
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
- Swords Dance
- Thunder Punch
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
<div class="pokemon-tab-panel" id="pokemon-tabs-bewear-panel-1">
Types: Normal / Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Fluffy
- Reckless
- Unnerve *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Ghost (0×)
- Dark (0.5×)

*Weak to*
- Fighting (2×)
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM08 - Bulk Up
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM04 - Strength

**Evolution Info**
Lv. 27

**Encounter Locations**
- Wakewater Isle — Grass (Day) (10%)
- Wakewater Isle — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="bewear" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">120</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Bind (Lv Evo)
- Tackle (Lv 1)
- Leer (Lv 1)
- Bide (Lv 5)
- Baby-Doll Eyes (Lv 10)
- Brutal Swing (Lv 14)
- Flail (Lv 19)
- Payback (Lv 23)
- Take Down (Lv 28)
- Hammer Arm (Lv 32)
- Slack Off (Lv 35)
- Thrash (Lv 37)
- Pain Split (Lv 41)
- Double-Edge (Lv 46)
- Superpower (Lv 50)

**Egg Moves**
- Ice Punch
- Thunder Punch
- Force Palm
- Endure
- Wide Guard
- Mega Kick
- Stomping Tantrum

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Ice Punch
- Mega Kick
- Mega Punch
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
- Swords Dance
- Thunder Punch
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
#pokemon-tabs-bewear-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-bewear-panel-0 { display: block; }
#pokemon-tabs-bewear-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-bewear-panel-1 { display: block; }
</style>
</details>
