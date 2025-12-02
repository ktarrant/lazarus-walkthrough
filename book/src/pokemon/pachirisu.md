<details class="pokemon-card-container">
<summary>Pachirisu (#210)</summary>
Types: Electric • Egg Groups: Fairy / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Fur Coat
- Pickup
- Volt Absorb *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Flying (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM58 - Thunder Wave
- HM01 - Cut
- HM05 - Flash

**Encounter Locations**
- Asfal Hills — Grass (Day) (10%)
- Pollen Road — Grass (Day) (10%)
- Port Pello — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="pachirisu" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-high">110</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-mid">455</span> |

**Level-Up Moves**
- Growl (Lv 1)
- Bide (Lv 1)
- Quick Attack (Lv 5)
- Charm (Lv 9)
- Spark (Lv 13)
- Endure (Lv 17)
- Nuzzle (Lv 19)
- Swift (Lv 21)
- Electro Ball (Lv 25)
- Sweet Kiss (Lv 29)
- Thunder Wave (Lv 33)
- Super Fang (Lv 37)
- Discharge (Lv 41)
- Last Resort (Lv 45)
- Hyper Fang (Lv 49)

**Egg Moves**
- Covet
- Bite
- Fake Tears
- Defense Curl
- Rollout
- Flatter
- Flail
- Iron Tail
- Tail Whip
- Follow Me
- Charge
- Bestow
- Ion Deluge
- Baby-Doll Eyes

**Tutor Moves**
- Defense Curl
- Endure
- Mud-Slap
- Rollout
- Sleep Talk
- Snore
- Swagger
- Swift
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
</details>
