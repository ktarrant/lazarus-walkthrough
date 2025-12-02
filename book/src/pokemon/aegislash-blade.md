<details class="pokemon-card-container">
<summary>Aegislash Blade (#151)</summary>
Types: Steel / Ghost • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Stance Change

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.25×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Ground (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- HM01 - Cut
- HM06 - Rock Smash

**Evolution Info**
Dusk Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="aegislash-blade" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-high">140</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-high">140</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- King's Shield (Lv Evo)
- Tackle (Lv 1)
- Swords Dance (Lv 1)
- Fury Cutter (Lv 5)
- Metal Sound (Lv 8)
- Pursuit (Lv 13)
- Autotomize (Lv 18)
- Shadow Sneak (Lv 20)
- Aerial Ace (Lv 22)
- Retaliate (Lv 26)
- Slash (Lv 29)
- Iron Defense (Lv 32)
- Night Slash (Lv 35)
- Power Trick (Lv 39)
- Iron Head (Lv 42)
- Phantom Force (Lv 45)
- Sacred Sword (Lv 47)
- Bitter Blade (Lv 50)
- Might Cleave (Lv 55)

**Egg Moves**
- Metal Sound
- Shadow Sneak
- Destiny Bond
- Wide Guard

**Tutor Moves**
- Endure
- Fury Cutter
- Rock Slide
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
</details>
