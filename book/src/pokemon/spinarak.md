<details class="pokemon-card-container">
<summary>Spinarak (#177)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-spinarak">
<input type="radio" name="pokemon-tabs-spinarak-group" id="pokemon-tabs-spinarak-tab-0" checked>
<label for="pokemon-tabs-spinarak-tab-0">Spinarak</label>
<input type="radio" name="pokemon-tabs-spinarak-group" id="pokemon-tabs-spinarak-tab-1">
<label for="pokemon-tabs-spinarak-tab-1">Ariados</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-spinarak-panel-0">
Types: Bug / Poison • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swarm
- Insomnia
- Sniper *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Fighting (0.25×)
- Poison (0.5×)
- Bug (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Psychic (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM22 - Solar Beam
- TM23 - Hex
- TM28 - Dig
- TM29 - Psychic
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- HM05 - Flash

**Encounter Locations**
- Acrisia Mountains (Peak) — Grass (Night) (20%)
- Erinys Path (East) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="spinarak" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-low">250</span> |

**Level-Up Moves**
- Poison Sting (Lv 1)
- String Shot (Lv 1)
- Constrict (Lv 1)
- Absorb (Lv 5)
- Infestation (Lv 8)
- Scary Face (Lv 12)
- Night Shade (Lv 15)
- Shadow Sneak (Lv 19)
- Bug Bite (Lv 22)
- Sucker Punch (Lv 26)
- Spider Web (Lv 29)
- Agility (Lv 33)
- Pin Missile (Lv 36)
- Cross Poison (Lv 40)
- Poison Jab (Lv 43)
- Cross Poison (Lv 47)
- Sticky Web (Lv 50)

**Egg Moves**
- Psybeam
- Disable
- Sonic Boom
- Baton Pass
- Pursuit
- Signal Beam
- Toxic Spikes
- Twineedle
- Electroweb
- Rage Powder
- Night Slash
- Megahorn
- Lunge

**Tutor Moves**
- Acid Spray
- Body Slam
- Double-Edge
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-spinarak-panel-1">
Types: Bug / Poison • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swarm
- Insomnia
- Sniper *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Fighting (0.25×)
- Poison (0.5×)
- Bug (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Psychic (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM22 - Solar Beam
- TM23 - Hex
- TM28 - Dig
- TM29 - Psychic
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- HM05 - Flash

**Evolution Info**
Lv. 22

**Encounter Locations**
- Areios Hideout — Grass (Day) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ariados" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">430</span> |

**Level-Up Moves**
- Swords Dance (Lv Evo)
- Focus Energy (Lv 1)
- Venom Drench (Lv 1)
- Fell Stinger (Lv 1)
- Bug Bite (Lv 1)
- Poison Sting (Lv 1)
- String Shot (Lv 1)
- Constrict (Lv 1)
- Absorb (Lv 5)
- Infestation (Lv 8)
- Scary Face (Lv 12)
- Night Shade (Lv 15)
- Shadow Sneak (Lv 19)
- Bug Bite (Lv 23)
- Sucker Punch (Lv 28)
- X-Scissor (Lv 30)
- Spider Web (Lv 32)
- Night Slash (Lv 32)
- Toxic Thread (Lv 35)
- Agility (Lv 37)
- Pin Missile (Lv 41)
- Cross Poison (Lv 44)
- Poison Jab (Lv 47)
- Phantom Force (Lv 50)
- Sticky Web (Lv 52)
- Toxic Thread (Lv 55)

**Egg Moves**
- Psybeam
- Disable
- Sonic Boom
- Baton Pass
- Pursuit
- Signal Beam
- Toxic Spikes
- Twineedle
- Electroweb
- Rage Powder
- Night Slash
- Megahorn
- Lunge

**Tutor Moves**
- Acid Spray
- Body Slam
- Double-Edge
- Endure
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
#pokemon-tabs-spinarak-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-spinarak-panel-0 { display: block; }
#pokemon-tabs-spinarak-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-spinarak-panel-1 { display: block; }
</style>
</details>
