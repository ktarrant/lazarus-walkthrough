<details class="pokemon-card-container">
<summary>Ninjask (#046)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-ninjask">
<input type="radio" name="pokemon-tabs-ninjask-group" id="pokemon-tabs-ninjask-tab-0">
<label for="pokemon-tabs-ninjask-tab-0">Nincada</label>
<input type="radio" name="pokemon-tabs-ninjask-group" id="pokemon-tabs-ninjask-tab-1" checked>
<label for="pokemon-tabs-ninjask-tab-1">Ninjask</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-ninjask-panel-0">
Types: Bug / Ground • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Compound Eyes
- Shed Skin
- Run Away *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Fighting (0.5×)
- Poison (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Water (2×)
- Ice (2×)
- Flying (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM37 - Sandstorm
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- HM01 - Cut
- HM05 - Flash

**Held Item**
Soft Sand

**Encounter Locations**
- Bronze Fields (North) — Grass (Day) (20%)
- Bronze Fields (South) — Grass (Day) (10%)
- Bronze Fields (South) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="nincada" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">31</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-low">266</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Harden (Lv 1)
- Absorb (Lv 5)
- Sand Attack (Lv 9)
- Fury Swipes (Lv 13)
- Mud-Slap (Lv 17)
- Metal Claw (Lv 21)
- Mind Reader (Lv 25)
- Bide (Lv 29)
- False Swipe (Lv 33)
- Dig (Lv 37)

**Egg Moves**
- Endure
- Feint Attack
- Gust
- Silver Wind
- Bug Buzz
- Night Slash
- Bug Bite
- Final Gambit

**Tutor Moves**
- Double-Edge
- Endure
- Fury Cutter
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ninjask-panel-1">
Types: Bug / Dark • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Speed Boost
- Sharpness
- Infiltrator *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ground (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Bug (2×)
- Rock (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM37 - Sandstorm
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM57 - Roost
- HM01 - Cut
- HM05 - Flash

**Evolution Info**
Lv. 20

**Encounter Locations**
- Sea of Asteri (East) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ninjask" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">61</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-high">160</span> |
| Total | <span class="stat-value stat-mid">466</span> |

**Level-Up Moves**
- Fury Cutter (Lv Evo)
- Screech (Lv Evo)
- Double Team (Lv Evo)
- Bug Bite (Lv 1)
- Scratch (Lv 1)
- Harden (Lv 1)
- Absorb (Lv 5)
- Sand Attack (Lv 9)
- Fury Swipes (Lv 13)
- Agility (Lv 17)
- Slash (Lv 23)
- Night Slash (Lv 25)
- Fell Stinger (Lv 27)
- Mind Reader (Lv 29)
- X-Scissor (Lv 33)
- Quiver Dance (Lv 35)
- Baton Pass (Lv 35)
- Knock Off (Lv 37)
- Swords Dance (Lv 41)
- Foul Play (Lv 43)
- Attack Order (Lv 47)
- Defend Order (Lv 47)
- Heal Order (Lv 47)

**Egg Moves**
- Endure
- Feint Attack
- Gust
- Silver Wind
- Bug Buzz
- Night Slash
- Bug Bite
- Final Gambit

**Tutor Moves**
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
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
#pokemon-tabs-ninjask-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-ninjask-panel-0 { display: block; }
#pokemon-tabs-ninjask-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-ninjask-panel-1 { display: block; }
</style>
</details>
