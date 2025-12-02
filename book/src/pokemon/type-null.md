<details class="pokemon-card-container">
<summary>Type: Null (#412)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-type-null">
<input type="radio" name="pokemon-tabs-type-null-group" id="pokemon-tabs-type-null-tab-0" checked>
<label for="pokemon-tabs-type-null-tab-0">Type: Null</label>
<input type="radio" name="pokemon-tabs-type-null-group" id="pokemon-tabs-type-null-tab-1">
<label for="pokemon-tabs-type-null-tab-1">Silvally</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-type-null-panel-0">
Types: Normal • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Battle Armor

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM37 - Sandstorm
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM58 - Thunder Wave
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="type-null" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">480</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Rage (Lv 5)
- Pursuit (Lv 10)
- Bite (Lv 13)
- Imprison (Lv 15)
- Aerial Ace (Lv 16)
- Crush Claw (Lv 20)
- Scary Face (Lv 23)
- X-Scissor (Lv 26)
- Take Down (Lv 30)
- Metal Sound (Lv 33)
- Iron Head (Lv 36)
- Double Hit (Lv 40)
- Air Slash (Lv 43)
- Punishment (Lv 46)
- Razor Wind (Lv 50)
- Tri Attack (Lv 53)
- Double-Edge (Lv 56)
- Heal Block (Lv 60)
- Parting Shot (Lv 65)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
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
<div class="pokemon-tab-panel" id="pokemon-tabs-type-null-panel-1">
Types: Normal • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- RKS System

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM30 - Shadow Ball
- TM32 - Double Team
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM40 - Aerial Ace
- TM42 - Facade
- TM43 - Poison Fang
- TM44 - Rest
- TM47 - Steel Wing
- TM55 - Snarl
- TM58 - Thunder Wave
- HM03 - Surf

**Evolution Info**
Lv. w/ High Friendship?
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="silvally" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-high">570</span> |

**Level-Up Moves**
- Multi-Attack (Lv Evo)
- Heal Block (Lv 1)
- Imprison (Lv 1)
- Iron Head (Lv 1)
- Poison Fang (Lv 1)
- Fire Fang (Lv 1)
- Ice Fang (Lv 1)
- Thunder Fang (Lv 1)
- Tackle (Lv 1)
- Rage (Lv 5)
- Pursuit (Lv 10)
- Bite (Lv 13)
- Aerial Ace (Lv 16)
- Crush Claw (Lv 20)
- Scary Face (Lv 23)
- X-Scissor (Lv 26)
- Take Down (Lv 30)
- Metal Sound (Lv 33)
- Iron Head (Lv 36)
- Double Hit (Lv 40)
- Air Slash (Lv 43)
- Punishment (Lv 46)
- Razor Wind (Lv 50)
- Tri Attack (Lv 53)
- Double-Edge (Lv 56)
- Heal Block (Lv 60)
- Parting Shot (Lv 65)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Explosion
- Icy Wind
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
#pokemon-tabs-type-null-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-type-null-panel-0 { display: block; }
#pokemon-tabs-type-null-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-type-null-panel-1 { display: block; }
</style>
</details>
