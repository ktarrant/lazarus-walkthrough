<details class="pokemon-card-container">
<summary>Meowscarada (#021)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-meowscarada">
<input type="radio" name="pokemon-tabs-meowscarada-group" id="pokemon-tabs-meowscarada-tab-0">
<label for="pokemon-tabs-meowscarada-tab-0">Sprigatito</label>
<input type="radio" name="pokemon-tabs-meowscarada-group" id="pokemon-tabs-meowscarada-tab-1">
<label for="pokemon-tabs-meowscarada-tab-1">Floragato</label>
<input type="radio" name="pokemon-tabs-meowscarada-group" id="pokemon-tabs-meowscarada-tab-2" checked>
<label for="pokemon-tabs-meowscarada-tab-2">Meowscarada</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-meowscarada-panel-0">
Types: Grass • Egg Groups: Field / Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Unnerve
- Protean *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>

**Encounter Locations**
- Sea of Vulcai — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="sprigatito" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-mid">61</span> |
| Defense | <span class="stat-value stat-mid">54</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">310</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=leafage">Leafage</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=hone-claws">Hone Claws</a> (Lv 10)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 13)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 15)
- <a href="move-lookup.html?q=seed-bomb">Seed Bomb</a> (Lv 17)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 21)
- <a href="move-lookup.html?q=worry-seed">Worry Seed</a> (Lv 25)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 28)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 32)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 36)

**Egg Moves**
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a>
- <a href="move-lookup.html?q=copycat">Copycat</a>
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a>
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-meowscarada-panel-1">
Types: Grass • Egg Groups: Field / Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Unnerve
- Protean *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>

**Evolution Info**
Lv. 16
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="floragato" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">61</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">63</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">63</span> |
| Speed | <span class="stat-value stat-mid">83</span> |
| Total | <span class="stat-value stat-mid">410</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=leafage">Leafage</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=hone-claws">Hone Claws</a> (Lv 10)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 13)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 15)
- <a href="move-lookup.html?q=seed-bomb">Seed Bomb</a> (Lv 20)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 24)
- <a href="move-lookup.html?q=worry-seed">Worry Seed</a> (Lv 28)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 33)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 38)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 42)
- <a href="move-lookup.html?q=leaf-storm">Leaf Storm</a> (Lv 46)

**Egg Moves**
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a>
- <a href="move-lookup.html?q=copycat">Copycat</a>
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a>
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-meowscarada-panel-2">
Types: Grass / Dark • Egg Groups: Field / Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Unnerve
- Protean *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Fighting (2×)
- Poison (2×)
- Flying (2×)
- Bug (4×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm31-brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm48-skill-swap">TM48 - Skill Swap</a>
- <a href="move-lookup.html?q=tm59-dark-pulse">TM59 - Dark Pulse</a>

**Evolution Info**
Lv. 36
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="meowscarada" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">76</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">81</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">123</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=flower-trick">Flower Trick</a> (Lv Evo)
- <a href="move-lookup.html?q=double-team">Double Team</a> (Lv 1)
- <a href="move-lookup.html?q=trick">Trick</a> (Lv 1)
- <a href="move-lookup.html?q=leafage">Leafage</a> (Lv 1)
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=hone-claws">Hone Claws</a> (Lv 10)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 13)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 15)
- <a href="move-lookup.html?q=seed-bomb">Seed Bomb</a> (Lv 20)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 24)
- <a href="move-lookup.html?q=worry-seed">Worry Seed</a> (Lv 29)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 33)
- <a href="move-lookup.html?q=night-slash">Night Slash</a> (Lv 38)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 40)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 43)
- <a href="move-lookup.html?q=knock-off">Knock Off</a> (Lv 46)
- <a href="move-lookup.html?q=grassy-terrain">Grassy Terrain</a> (Lv 50)
- <a href="move-lookup.html?q=leaf-storm">Leaf Storm</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a>
- <a href="move-lookup.html?q=copycat">Copycat</a>
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a>
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
- <a href="move-lookup.html?q=thunder-punch">Thunder Punch</a>
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
#pokemon-tabs-meowscarada-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-meowscarada-panel-0 { display: block; }
#pokemon-tabs-meowscarada-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-meowscarada-panel-1 { display: block; }
#pokemon-tabs-meowscarada-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-meowscarada-panel-2 { display: block; }
</style>
</details>
