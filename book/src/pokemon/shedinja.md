<details class="pokemon-card-container">
<summary>Shedinja (#047)</summary>
Types: Bug / Ghost • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Wonder Guard

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Ground (0.5×)
- Bug (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM23 - Hex
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM37 - Sandstorm
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM51 - Will-O-Wisp
- HM01 - Cut
- HM05 - Flash

**Evolution Info**
Lv. 20, empty party slot, PokéBall

**Encounter Locations**
- Sea of Asteri (East) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="shedinja" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">1</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-low">236</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Harden (Lv 1)
- Absorb (Lv 5)
- Sand Attack (Lv 9)
- Fury Swipes (Lv 13)
- Spite (Lv 17)
- Shadow Sneak (Lv 21)
- Mind Reader (Lv 25)
- Confuse Ray (Lv 29)
- Shadow Claw (Lv 33)
- Fell Stinger (Lv 35)
- Grudge (Lv 37)
- Heal Block (Lv 41)
- Phantom Force (Lv 45)
- Attack Order (Lv 47)
- Defend Order (Lv 47)

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
- Dream Eater
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
</details>
