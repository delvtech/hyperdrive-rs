# Hyperdrive Protocol

## Overview

Hyperdrive is a new AMM that allows users to open long and short positions to get exposure to fixed and variable rates respectively. Terms are "Mint on Demand" and the AMM essentially underwrites a new term for the user when they enter a position. Long traders receive fixed interest during the term and give up any variable interest collected on their investment. Short traders sell long positions to the LP which means that they pay fixed interest during the term and receive all of the variable interest collected on the LP's investment. LPs provide liquidity to a single pool, which underwrites a variety of fixed terms with differing maturity dates. This removes fragmented liquidity and gives LPs an everlasting exposure to the markets.



## Flat + Curve

In Hyperdrive, trades only take place on a single invariant.  When a user opens a new position, the trade is made on a curve that prices the asset as a function of the reserves and a **fixed** time until maturity. When a user closes a position, we price the trade by splitting into to components: new bonds and matured bonds.  The new bonds are priced on the curve and matured bonds can be redeemed 1:1 with the base asset.  For example, let's say Alice wants to sell 12 bonds that are 9 months from maturity on a 12 month term.  We would consider 3 bonds mature and offer a 1:1 redemption for them and the remaining 9 bonds would be priced on the curve with the full time remaining until maturity.

## Backdating

If every position is opened with a fixed position duration (e.g. 6 months from the current block timestamp), LPs would need to run keeper bots to close matured shorts as the shorts would continue accruing interest after maturity. This would create a situation in which LPs must run off-chain infrastructure or pay a portion of their proceeds to keeper bots.

### Checkpointing

To reduce the amount of keeping required to maintain the accounting system, we introduce a checkpointing system that groups long and short positions by the time at which they were opened. These checkpoints store the share price of the first trade in the checkpoint. This price is used to compute the returns of long and short positions that mature in that checkpoint.

Mechanically, checkpointing backdates all of the positions opened within the checkpoint interval to the start of the checkpoint. This means that in most cases traders will benefit from interest that has already been collected, so the traders may have to pay slightly more when opening their position. 

### Calculating Maturity Time

Suppose that the current time is $t$, the contract's checkpoint duration is $d_c$, and the contract's position duration is $d$. The position's start date will be $t_s = t - (t \space \mod \space d_c)$ and the position's maturity date will be $t_m = t_s + d$.



## Pricing

The following sections describe the accounting logic needed for the pricing functions. This logic is agnostic to the invariant used and so we represent the invariant function with $I()$. Right now, our implementation uses the invariant described in [YieldSpace with Yield Bearing Vaults](https://hackmd.io/@DFwMpOvuQ_e2wVXLeKnTuw/BJfLJlcnF).  The following list represents the variations we use and their definitions:

- $I_{BondsOutSharesIn}$ is defined [here](https://hackmd.io/@DFwMpOvuQ_e2wVXLeKnTuw/BJfLJlcnF#fyTokenOutForSharesIn)
- $I_{SharesOutBondsIn}$ is defined [here](https://hackmd.io/@DFwMpOvuQ_e2wVXLeKnTuw/BJfLJlcnF#sharesOutForFYTokenIn)
- $I_{SharesInBondsOut}$ is defined [here](https://hackmd.io/@DFwMpOvuQ_e2wVXLeKnTuw/BJfLJlcnF#sharesInForFYTokenOut)


### Open Long

The trader supplies $\Delta x$ base and receives $\Delta y$ bonds. At the current share price $c$, $\Delta x = c \cdot \Delta z$ where $\Delta z$ is the amount of shares being traded with $t$ time remaining in the term.

$$
\begin{aligned}
\Delta y_{flat}' = \Delta z \cdot (1 - t)\\
\Delta y_{curve}' = I_{BondsOutSharesIn}(\Delta z \cdot t)
\end{aligned}
$$

> Note: that $\Delta y_{flat}'$ is only zero if the position's start time is EXACTLY the same as the most recent checkpoint.

Next, we account for fees:

$$
\begin{aligned}
\Delta y_{flat} = \Delta y_{flat}'-c \cdot \Delta z \cdot (1 - t) \cdot \phi_{flat}\\
\Delta y_{curve} = \Delta y_{curve}' - (\frac{1}{p} - 1) \cdot c \cdot \Delta z \cdot t \cdot \phi_{curve}\\
\Delta y = \Delta y_{flat} + \Delta y_{curve}
\end{aligned}
$$

where $\phi_{flat}$ is the flat fee, $\phi_{curve}$ is the fee for the curve and $p$ is the spot price as defined in [YieldSpace with Yield Bearing Vaults](https://hackmd.io/@DFwMpOvuQ_e2wVXLeKnTuw/BJfLJlcnF). The reserves are updated as follows:

$$
\begin{aligned}
z_{reserves} = z_{reserves} + \Delta z \\
y_{reserves} = y_{reserves} - \Delta y_{curve}
\end{aligned}
$$

> Note: We only debit $y_{curve}$ from the $y_{reserves}$ because the $y_{flat}$ portion is considered mature and should have no impact on the fixed rate market.

$o_l$, accounts for outstanding long positions to ensure that the AMM has enough funds to honor those positions when they close and must also be updated.

$$
o_l = o_l + \Delta y
$$

Since the base buffer may have increased relative to the base reserves and the bond reserves decreased, we must ensure that the base reserves are greater than the number of outstanding long positions. The following invariant must be preserved when opening a long:

$$
c \cdot z_{reserves} \ge o_l
$$


> TODO: Cover the aggregates accounting

Update the long's average maturity time, $t_l$
Update the long's base volume, $v_l$


### Close Long

The trader closes their long position of $\Delta y$ bonds for $\Delta x$ base with $t$ time remaining in the term:

$$
\begin{aligned}
\Delta z_{flat}' = \frac{\Delta y \cdot (1 - t)}{c} \\
\Delta z_{curve}' = I_{SharesOutBondsIn}(\Delta y \cdot t)
\end{aligned}
$$

Next, we account for fees:

$$
\begin{aligned}
\Delta z_{flat} = \Delta z_{flat}' - \frac{\Delta y \cdot (1 - t) \cdot \phi_{flat}}{c}\\
\Delta z_{curve} = \Delta z_{curve}' - \frac{(1 - p) \cdot \Delta y \cdot t \cdot \phi_{curve}}{c}\\
\Delta z = \Delta z_{flat} + \Delta z_{curve}
\end{aligned}
$$

The number of outstanding longs are reduced by $\Delta y$:

$$
o_l = o_l - \Delta y
$$

The reserves are updated:

$$
\begin{aligned}
z_{reserves} = z_{reserves} - \Delta z\\
y_{reserves} = y_{reserves} + \Delta y_{curve}
\end{aligned}
$$


If there are long withdrawal shares outstanding, $w_{l}$, then we must also update the z reserves by crediting the amount of variable interest earned by the LPs.

$$
z_{reserves} = z_{reserves} + (\frac{\Delta y}{c_0} - \Delta z) \cdot \frac{\min(w_{l}, \Delta y)}{\Delta y}
$$

The $y_{reserves}$ are recalculated to ensure that the apr doesn't change from before the redemption.

The number of long withdrawal shares outstanding, $w_{l}$, is decreased by:

$$
w_{l} = w_{l} - \frac{\min(w_{l}, \Delta y)}{\Delta y}
$$

> Note: Long withdrawal shares receive the same proceeds as shorts because they **are** shorts without the ability to close early.

The number of long withdrawal share proceeds, $p_{l}$, is increased by:

$$
p_{l} = p_{l} + (\frac{\Delta y}{c_0} - \Delta z) \cdot \frac{\min(w_{l}, \Delta y)}{\Delta y}
$$

Update the long's average maturity time, $t_l$
Update the long's base volume, $v_l$

> TODO: Cover the aggregates accounting

### Open Short

The trader shorts $\Delta y$ bonds with $\Delta x$ base with $t$ time remaining in the term. To calculate the short we must determine how many $\Delta z$ shares that $\Delta y$ bonds are worth right now. 

$$
\begin{aligned}
\Delta z_{flat}' = \frac{\Delta y \cdot (1 - t)}{c}\\
\Delta z_{curve}' = I_{SharesOutBondsIn}(\Delta y \cdot t)
\end{aligned}
$$

Next, we account for fees:

$$
\begin{aligned}
\Delta z_{flat} = \Delta z_{flat}' - \frac{\Delta y \cdot (1 - t) \cdot \phi_{flat}}{c}\\
\Delta z_{curve} = \Delta z_{curve}' - \frac{(1 - p) \cdot \Delta y \cdot t \cdot \phi_{curve}}{c}
\end{aligned}
$$

Summing the flat and curve portion together

$$
\Delta z = \Delta z_{flat} + \Delta z_{curve}
$$

gives us the $\Delta z$ shares that the $\Delta y$ bonds are worth. Next, we calculate the $\Delta x$ base that the user owes when opening the short. To do this, we calculate the max loss:

$$
x_{maxloss} = \Delta y - c_1 \cdot \Delta z
$$

and the interest owed from backdating to the most recent checkpoint:

$$
x_{interest} = (\frac{c_1}{c_0} - 1) \cdot \Delta y
$$

The $\Delta x$ base that the user must provide to open the short is equal to the sum of the max loss and the interest owed from backdating to the most recent checkpoint:

$$
\Delta x = x_{maxloss} + x_{interest}
$$

The reserves are updated as follows:

$$
\begin{aligned}
z_{reserves} = z_{reserves} - \Delta z\\
y_{reserves} = y_{reserves} + \Delta y
\end{aligned}
$$

Since the share reserves are reduced, we need to verify that the base reserves are greater than or equal to the number of outstanding long positions. The following invariant must be preserved when opening a short:

$$
c \cdot z_{reserves} \ge o_l
$$


The number of outstanding shorts, $o_s$, are increased by $\Delta y$:

$$
o_s = o_s + \Delta y
$$

> TODO: Cover the aggregates accounting

Update the short's average maturity time, $t_s$
Update the short's base volume, $v_s$

### Close Short

The trader closes their short position of $\Delta y$ bonds for $\Delta x$ base with $t$ time remaining in the term:

$$
\begin{aligned}
\Delta z_{flat}' = \frac{\Delta y \cdot (1 - t)}{c} \\
\Delta z_{curve}' = I_{SharesInBondsOut}(\Delta y \cdot t)
\end{aligned}
$$

Next, we account for fees:

$$
\begin{aligned}
\Delta z_{flat} = \Delta z_{flat}' + \frac{\Delta y \cdot (1 - t) \cdot \phi_{flat}}{c}\\
\Delta z_{curve} = \Delta z_{curve}' + \frac{(1 - p) \cdot \Delta y \cdot t \cdot \phi_{curve}}{c}\\
\Delta z = \Delta z_{flat} + \Delta z_{curve}
\end{aligned}
$$

$\Delta z$ represents the number of shares the user would need to purchase back the bonds and close the short.

The number of outstanding shorts, $o_s$, are decreased by $\Delta y$:

$$
o_s = o_s - \Delta y
$$

The reserves are updated as follows:

$$
\begin{aligned}
z_{reserves} = z_{reserves} + \Delta z\\
y_{reserves} = y_{reserves} - \Delta y_{curve}
\end{aligned}
$$

If there are short withdrawal shares outstanding, $w_{s}$, then we update the reserves to reflect the proportion of shares that are owed the LPs vs the withdrawal pool.

$$
z_{reserves} = z_{reserves} - \Delta z \cdot \frac{\min(w_{s}, \Delta y)}{\Delta y}\\
$$


The $y_{reserves}$ are recalculated to ensure that the apr doesn't change from before the short close.

The number of short withdrawal shares outstanding, $w_s$, is decreased by:

$$
w_{s} = w_{s} - \frac{\min(w_{s}, \Delta y)}{\Delta y}
$$

> Note: Short withdrawal shares receive the same proceeds as longs because they **are** longs.

The number of short withdrawal share proceeds, $p_{s}$, is increased by:

$$
p_{s} = p_{s} + \Delta z \cdot\frac{\min(w_{s}, \Delta y)}{\Delta y}
$$

> TODO: Cover aggregates accounting

Update the short's average maturity time, $t_s$
Update the short's base volume, $v_s$

### Add Liquidity

User deposits $\Delta x$ base at share price c where $\frac{\Delta x}{c} = \Delta z$ and $l$ is the total supply of lp shares and receives $\Delta l$ lp shares.

$$
\Delta l = \frac{\Delta z \cdot l}{z_{reserves} + a_s - a_l}
$$

The short adjustment, $a_s$, is:

$$
a_s = v_s \cdot t + o_s \cdot (1 - t_s)
$$

The long adjustment, $a_l$, is:

$$
a_l = v_l \cdot t + o_l \cdot (1 - t_l)
$$

The reserves are updated as follows:

$$
z_{reserves} = z_{reserves} + \Delta z
$$

The $y_{reserves}$ are recalculated to ensure that the apr doesn't change from before the lp added liquidity.

### Remove Liquidity

User redeems $\Delta l$ lp shares and receives $\Delta x$ base at share price c where $\frac{\Delta x}{c} = \Delta z$. The user also receives $\Delta w_{s}$ short withdrawal shares and $\Delta w_{l}$ long withdrawal shares.

$$
\begin{aligned}
\Delta x = (z_{reserves} - \frac{o_l}{c}) \cdot \frac{\Delta l}{l}\\
\Delta w_l = l_o * \frac{\Delta l}{l}\\
\Delta w_s = s_o * \frac{\Delta l}{l}
\end{aligned}
$$

The reserves are updated as follows:

$$
z_{reserves} = z_{reserves} - \Delta z
$$

The $y_{reserves}$ are recalculated to ensure that the apr doesn't change from before the lp added liquidity.

The withdrawal shares are updated as follows:

$$
\begin{aligned}
w_l = w_l + \Delta w_l\\
w_s = w_s + \Delta w_s
\end{aligned}
$$