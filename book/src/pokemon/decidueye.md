<details class="pokemon-card-container">
<summary>Decidueye (#003)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-decidueye">
<input type="radio" name="pokemon-tabs-decidueye-group" id="pokemon-tabs-decidueye-tab-0">
<label for="pokemon-tabs-decidueye-tab-0">Rowlet</label>
<input type="radio" name="pokemon-tabs-decidueye-group" id="pokemon-tabs-decidueye-tab-1">
<label for="pokemon-tabs-decidueye-tab-1">Dartrix</label>
<input type="radio" name="pokemon-tabs-decidueye-group" id="pokemon-tabs-decidueye-tab-2">
<label for="pokemon-tabs-decidueye-tab-2">Hisuian Decidueye</label>
<input type="radio" name="pokemon-tabs-decidueye-group" id="pokemon-tabs-decidueye-tab-3" checked>
<label for="pokemon-tabs-decidueye-tab-3">Decidueye</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-decidueye-panel-0">
Types: Grass / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Long Reach
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Grass (0.25×)
- Fighting (0.5×)
- Ground (0×)

*Weak to*
- Fire (2×)
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM57 - Roost

**Encounter Locations**
- Sea of Vulcai — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="rowlet" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">68</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">42</span> |
| Total | <span class="stat-value stat-mid">320</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leafage (Lv 1)
- Growl (Lv 4)
- Peck (Lv 8)
- Astonish (Lv 11)
- Razor Leaf (Lv 14)
- Ominous Wind (Lv 16)
- Foresight (Lv 18)
- Pluck (Lv 22)
- Synthesis (Lv 25)
- Swords Dance (Lv 29)
- Sucker Punch (Lv 32)
- Dual Wingbeat (Lv 35)
- Leaf Blade (Lv 37)
- Feather Dance (Lv 39)
- Brave Bird (Lv 43)
- Nasty Plot (Lv 46)
- U-Turn (Lv 49)

**Egg Moves**
- Curse
- Confuse Ray
- Ominous Wind
- Haze
- Baton Pass
- Defog

**Tutor Moves**
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-decidueye-panel-1">
Types: Grass / Flying • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Long Reach
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Grass (0.25×)
- Fighting (0.5×)
- Ground (0×)

*Weak to*
- Fire (2×)
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM57 - Roost

**Evolution Info**
Lv. 17
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dartrix" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">78</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">52</span> |
| Total | <span class="stat-value stat-mid">420</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Leafage (Lv 1)
- Growl (Lv 4)
- Peck (Lv 8)
- Astonish (Lv 11)
- Razor Leaf (Lv 14)
- Ominous Wind (Lv 16)
- Foresight (Lv 18)
- Pluck (Lv 22)
- Synthesis (Lv 25)
- Swords Dance (Lv 29)
- Sucker Punch (Lv 32)
- Dual Wingbeat (Lv 35)
- Leaf Blade (Lv 37)
- Feather Dance (Lv 39)
- Brave Bird (Lv 43)
- Nasty Plot (Lv 46)
- U-Turn (Lv 49)

**Egg Moves**
- Curse
- Confuse Ray
- Ominous Wind
- Haze
- Baton Pass
- Defog

**Tutor Moves**
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-decidueye-panel-2">
Types: Grass / Fighting • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Strong Legs
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (4×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM08 - Bulk Up
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM12 - Taunt
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM57 - Roost
- HM06 - Rock Smash

**Evolution Info**
Lv. 34, Day
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-decidueye" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">93</span> |
| Attack | <span class="stat-value stat-high">122</span> |
| Defense | <span class="stat-value stat-mid">81</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-mid">64</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Rock Smash (Lv Evo)
- Jump Kick (Lv 1)
- Leaf Storm (Lv 1)
- U-Turn (Lv 1)
- Mach Punch (Lv 1)
- Tackle (Lv 1)
- Leafage (Lv 1)
- Growl (Lv 4)
- Peck (Lv 8)
- Astonish (Lv 11)
- Razor Leaf (Lv 14)
- Ominous Wind (Lv 16)
- Foresight (Lv 18)
- Pluck (Lv 22)
- Synthesis (Lv 25)
- Swords Dance (Lv 29)
- Sucker Punch (Lv 32)
- Dual Wingbeat (Lv 35)
- Leaf Blade (Lv 37)
- Feather Dance (Lv 39)
- Triple Arrows (Lv 41)
- Brave Bird (Lv 45)
- Trop Kick (Lv 48)
- Triple Axel (Lv 52)

**Egg Moves**
- Curse
- Confuse Ray
- Ominous Wind
- Haze
- Baton Pass
- Defog

**Tutor Moves**
- Endure
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-decidueye-panel-3">
Types: Grass / Ghost • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Overgrow
- Long Reach
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Flying (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM23 - Hex
- TM30 - Shadow Ball
- TM32 - Double Team
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM57 - Roost

**Evolution Info**
Lv. 34, Night
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="decidueye" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">78</span> |
| Attack | <span class="stat-value stat-high">107</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">94</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-mid">76</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Spirit Shackle (Lv Evo)
- Phantom Force (Lv 1)
- Leaf Storm (Lv 1)
- U-Turn (Lv 1)
- Shadow Sneak (Lv 1)
- Tackle (Lv 1)
- Leafage (Lv 1)
- Growl (Lv 4)
- Peck (Lv 8)
- Astonish (Lv 11)
- Razor Leaf (Lv 14)
- Ominous Wind (Lv 16)
- Foresight (Lv 18)
- Pluck (Lv 22)
- Synthesis (Lv 25)
- Swords Dance (Lv 29)
- Sucker Punch (Lv 32)
- Dual Wingbeat (Lv 35)
- Leaf Blade (Lv 37)
- Feather Dance (Lv 39)
- Shadow Sneak (Lv 41)
- Brave Bird (Lv 45)
- Nasty Plot (Lv 48)
- U-Turn (Lv 52)

**Egg Moves**
- Curse
- Confuse Ray
- Ominous Wind
- Haze
- Baton Pass
- Defog

**Tutor Moves**
- Endure
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
#pokemon-tabs-decidueye-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-decidueye-panel-0 { display: block; }
#pokemon-tabs-decidueye-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-decidueye-panel-1 { display: block; }
#pokemon-tabs-decidueye-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-decidueye-panel-2 { display: block; }
#pokemon-tabs-decidueye-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-decidueye-panel-3 { display: block; }
</style>
</details>
