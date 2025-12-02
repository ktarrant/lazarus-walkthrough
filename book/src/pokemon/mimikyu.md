<details class="pokemon-card-container">
<summary>Mimikyu (#251)</summary>
Types: Ghost / Fairy • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Disguise

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Bug (0.25×)
- Dragon (0×)

*Weak to*
- Ghost (2×)
- Steel (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM51 - Will-O-Wisp
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse

**Held Item**
Chesto Berry

**Encounter Locations**
- Erinys Path (East) — Grass (Day) (2%)
- Erinys Path (East) — Grass (Night) (2%)
- Port Pello — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mimikyu" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-high">105</span> |
| Speed | <span class="stat-value stat-high">96</span> |
| Total | <span class="stat-value stat-mid">486</span> |

**Level-Up Moves**
- Wood Hammer (Lv 1)
- Splash (Lv 1)
- Scratch (Lv 1)
- Astonish (Lv 1)
- Copycat (Lv 1)
- Double Team (Lv 5)
- Baby-Doll Eyes (Lv 10)
- Shadow Sneak (Lv 14)
- Mimic (Lv 18)
- Feint Attack (Lv 21)
- Covet (Lv 24)
- Charm (Lv 28)
- Slash (Lv 32)
- Shadow Claw (Lv 35)
- Crush Claw (Lv 38)
- Hone Claws (Lv 41)
- Play Rough (Lv 44)
- Pain Split (Lv 47)
- Spirit Shackle (Lv 50)
- Dire Claw (Lv 60)

**Egg Moves**
- Grudge
- Destiny Bond
- Curse
- Nightmare

**Tutor Moves**
- Dream Eater
- Endure
- Psych Up
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
