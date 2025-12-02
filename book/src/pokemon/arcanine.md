<details class="pokemon-card-container">
<summary>Arcanine (#377)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-arcanine">
<input type="radio" name="pokemon-tabs-arcanine-group" id="pokemon-tabs-arcanine-tab-0">
<label for="pokemon-tabs-arcanine-tab-0">Growlithe</label>
<input type="radio" name="pokemon-tabs-arcanine-group" id="pokemon-tabs-arcanine-tab-1" checked>
<label for="pokemon-tabs-arcanine-tab-1">Arcanine</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-arcanine-panel-0">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Flash Fire
- Fur Coat *(Hidden)*

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
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM28 - Dig
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM51 - Will-O-Wisp
- TM55 - Snarl
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Jusmail Town — Grass (Day) (4%)
- Marmaro Island — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="growlithe" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">350</span> |

**Level-Up Moves**
- Bite (Lv 1)
- Roar (Lv 1)
- Ember (Lv 6)
- Leer (Lv 8)
- Odor Sleuth (Lv 10)
- Helping Hand (Lv 12)
- Flame Wheel (Lv 17)
- Reversal (Lv 19)
- Fire Fang (Lv 21)
- Take Down (Lv 23)
- Flame Burst (Lv 28)
- Agility (Lv 30)
- Retaliate (Lv 32)
- Flamethrower (Lv 34)
- Crunch (Lv 39)
- Heat Wave (Lv 41)
- Outrage (Lv 43)
- Flare Blitz (Lv 45)

**Egg Moves**
- Body Slam
- Crunch
- Thrash
- Fire Spin
- Howl
- Heat Wave
- Double-Edge
- Flare Blitz
- Morning Sun
- Covet
- Iron Tail
- Double Kick
- Close Combat
- Burn Up

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-arcanine-panel-1">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Flash Fire
- Fur Coat *(Hidden)*

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
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM22 - Solar Beam
- TM28 - Dig
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM51 - Will-O-Wisp
- TM55 - Snarl
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Fire Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="arcanine" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-high">555</span> |

**Level-Up Moves**
- Extreme Speed (Lv Evo)
- Thunder Fang (Lv 1)
- Bite (Lv 1)
- Roar (Lv 1)
- Ember (Lv 6)
- Leer (Lv 8)
- Odor Sleuth (Lv 10)
- Helping Hand (Lv 12)
- Flame Wheel (Lv 17)
- Reversal (Lv 19)
- Fire Fang (Lv 21)
- Take Down (Lv 23)
- Flame Burst (Lv 28)
- Agility (Lv 30)
- Retaliate (Lv 32)
- Flamethrower (Lv 34)
- Crunch (Lv 39)
- Heat Wave (Lv 41)
- Outrage (Lv 43)
- Flare Blitz (Lv 45)

**Egg Moves**
- Body Slam
- Crunch
- Thrash
- Fire Spin
- Howl
- Heat Wave
- Double-Edge
- Flare Blitz
- Morning Sun
- Covet
- Iron Tail
- Double Kick
- Close Combat
- Burn Up

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Mud-Slap
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
#pokemon-tabs-arcanine-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-arcanine-panel-0 { display: block; }
#pokemon-tabs-arcanine-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-arcanine-panel-1 { display: block; }
</style>
</details>
